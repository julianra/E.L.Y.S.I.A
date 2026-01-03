// ======================================================================
// 📍 FILE: elysia_core/src/http/install.rs
// 📝 ROLE:
//   Installeert module-zips automatisch vanuit uploads
//   - Valideert ZIP-structuur (module.json + dist/ + runtime/)
//   - Extract naar AppData/Local/elysia/modules/<module-id>
//   - Verwijdert ZIP na succesvolle install
// ======================================================================

use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use std::{
    collections::HashSet,
    fs,
    io::Read,
    path::{Path as FsPath, PathBuf},
};
use zip::ZipArchive;

use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;


// --------------------------------------------------
// Paths (CANONICAL)
// --------------------------------------------------

fn data_root() -> Result<PathBuf, StatusCode> {
    let base = dirs::data_local_dir().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(base.join("elysia"))
}

fn uploads_modules_dir() -> Result<PathBuf, StatusCode> {
    Ok(data_root()?.join("uploads").join("modules"))
}

fn modules_dir() -> Result<PathBuf, StatusCode> {
    Ok(data_root()?.join("modules"))
}

// --------------------------------------------------
// ZIP structure validation
// --------------------------------------------------

fn validate_zip_structure(zip: &mut ZipArchive<std::fs::File>) -> Result<(), StatusCode> {
    let mut roots: HashSet<String> = HashSet::new();
    let mut has_manifest = false;
    let mut has_dist = false;
    let mut has_runtime = false;

    for i in 0..zip.len() {
        let entry = zip.by_index(i).map_err(|_| StatusCode::BAD_REQUEST)?;
        let name = entry.name();

        let root = name.split('/').next().unwrap_or("");
        if root.is_empty() {
            continue;
        }

        roots.insert(root.to_string());

        match root {
            "module.json" if !entry.is_dir() => has_manifest = true,
            "dist" if entry.is_dir() => has_dist = true,
            "runtime" if entry.is_dir() => has_runtime = true,
            _ => {}
        }
    }

    if roots.len() != 3 || !has_manifest || !has_dist || !has_runtime {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    Ok(())
}

// --------------------------------------------------
// module.json → module-id
// --------------------------------------------------

fn read_module_id(zip_path: &FsPath) -> Result<String, StatusCode> {
    let file = fs::File::open(zip_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut zip = ZipArchive::new(file).map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut manifest = zip
        .by_name("module.json")
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    let mut json = String::new();
    manifest
        .read_to_string(&mut json)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let v: serde_json::Value =
        serde_json::from_str(&json).map_err(|_| StatusCode::BAD_REQUEST)?;

    let id = v
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::UNPROCESSABLE_ENTITY)?;

    if !id
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    Ok(id.to_string())
}

// --------------------------------------------------
// Extract ZIP → module directory
// --------------------------------------------------

fn extract_zip(zip_path: &FsPath, module_id: &str) -> Result<(), StatusCode> {
    let modules = modules_dir()?;
    fs::create_dir_all(&modules).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let dest = modules.join(module_id);
    let tmp = modules.join(format!(".tmp-{}", module_id));

    if dest.exists() {
        return Ok(());
    }

    if tmp.exists() {
        let _ = fs::remove_dir_all(&tmp);
    }

    fs::create_dir_all(&tmp).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let file = fs::File::open(zip_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut zip = ZipArchive::new(file).map_err(|_| StatusCode::BAD_REQUEST)?;

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|_| StatusCode::BAD_REQUEST)?;

        let outpath = match entry.enclosed_name() {
            Some(p) => tmp.join(p),
            None => continue,
        };

        if entry.is_dir() {
            fs::create_dir_all(&outpath).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            }

            let mut outfile =
                fs::File::create(&outpath).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            std::io::copy(&mut entry, &mut outfile)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    fs::rename(&tmp, &dest).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

// --------------------------------------------------
// HTTP endpoint (ook intern gebruikt)
// --------------------------------------------------

pub async fn install_module(Path(upload_id): Path<String>) -> impl IntoResponse {
    let upload_id = upload_id.to_lowercase();

    let zip_path = match uploads_modules_dir() {
        Ok(d) => d.join(format!("{}.zip", upload_id)),
        Err(code) => return code,
    };

    log::info!(
        "[CORE][INSTALL] Looking for ZIP at {}",
        zip_path.display()
    );

    if !zip_path.exists() {
        log::warn!(
            "[CORE][INSTALL] ZIP not found for {}",
            upload_id
        );
        return StatusCode::NOT_FOUND;
    }

    // 1. structuur-validatie
    {
        let file = match fs::File::open(&zip_path) {
            Ok(f) => f,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        };

        let mut zip = match ZipArchive::new(file) {
            Ok(z) => z,
            Err(_) => return StatusCode::BAD_REQUEST,
        };

        fn validate_zip_structure(zip: &mut ZipArchive<std::fs::File>) -> Result<(), StatusCode> {
    let mut roots: HashSet<String> = HashSet::new();
    let mut has_manifest = false;
    let mut has_dist = false;
    let mut has_runtime = false;

    for i in 0..zip.len() {
        let entry = zip.by_index(i).map_err(|_| StatusCode::BAD_REQUEST)?;
        let name = entry.name();

        // Root element (eerste padcomponent)
        let root = name.split('/').next().unwrap_or("");
        if root.is_empty() {
            continue;
        }

        roots.insert(root.to_string());

        // module.json exact op root
        if name == "module.json" {
            has_manifest = true;
        }

        // dist/* of dist/
        if name.starts_with("dist/") {
            has_dist = true;
        }

        // runtime/* of runtime/
        if name.starts_with("runtime/") {
            has_runtime = true;
        }
    }

    if roots.len() != 3 {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    if !has_manifest || !has_dist || !has_runtime {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    Ok(())
}

    }

    // 2. module-id lezen
    let module_id = match read_module_id(&zip_path) {
        Ok(id) => id,
        Err(code) => return code,
    };

    // 3. extract
    if extract_zip(&zip_path, &module_id).is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    // 4. ZIP verwijderen
    match fs::remove_file(&zip_path) {
    Ok(_) => {
        log::info!(
            "[CORE][INSTALL] Cleaned up upload ZIP {}",
            zip_path.display()
        );
    }
    Err(e) => {
        log::warn!(
            "[CORE][INSTALL] Failed to remove upload ZIP {}: {}",
            zip_path.display(),
            e
        );
    }
}

    log::info!(
        "[CORE][INSTALL] Module installed | id={} source={}",
        module_id,
        upload_id
    );

    StatusCode::CREATED
}
