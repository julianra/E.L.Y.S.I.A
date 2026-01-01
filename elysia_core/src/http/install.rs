// ======================================================================
// 📍 FILE: elysia_core/src/http/install.rs
// 📝 ROLE:
//   Installeert een reeds geüploade module-zip
//   - Admin-only (via guard)
//   - Extract naar plugins/
//   - Geen execution
// ======================================================================

use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use std::{fs, path::PathBuf};
use zip::ZipArchive;

fn data_root() -> Result<PathBuf, StatusCode> {
    let base = dirs::data_local_dir().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(base.join("Elysia"))
}

fn uploads_modules_dir() -> Result<PathBuf, StatusCode> {
    Ok(data_root()?.join("uploads").join("modules"))
}

pub async fn install_module(Path(upload_id): Path<String>) -> impl IntoResponse {
    let upload_id = upload_id.to_lowercase();

    // Uploads staan in data_local/Elysia/uploads/modules/<id>.zip
    let zip_path = match uploads_modules_dir() {
        Ok(d) => d.join(format!("{}.zip", upload_id)),
        Err(code) => return code,
    };

    if !zip_path.exists() {
        return StatusCode::NOT_FOUND;
    }

    let file = match fs::File::open(&zip_path) {
        Ok(f) => f,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let mut zip = match ZipArchive::new(file) {
        Ok(z) => z,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    // plugin-id = upload_id (later uit manifest)
    let plugin_id = upload_id;

    // Voor Fase 1: plugins/ blijft zoals loader gebruikt (relatief)
    // (We verplaatsen dit pas als jij beslist dat plugins ook onder data_local moet vallen)
    let dest = PathBuf::from("plugins").join(&plugin_id);

    if dest.exists() {
        return StatusCode::CONFLICT;
    }

    if fs::create_dir_all(&dest).is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    for i in 0..zip.len() {
        let mut entry = match zip.by_index(i) {
            Ok(e) => e,
            Err(_) => return StatusCode::BAD_REQUEST,
        };

        let outpath = match entry.enclosed_name() {
            Some(p) => dest.join(p),
            None => continue, // zip-slip bescherming
        };

        if entry.is_dir() {
            fs::create_dir_all(&outpath).ok();
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent).ok();
            }

            let mut outfile = match fs::File::create(&outpath) {
                Ok(f) => f,
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            };

            if std::io::copy(&mut entry, &mut outfile).is_err() {
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
    }

    log::info!(
        "[CORE][INSTALL] Installed module | upload_id={} dest={}",
        plugin_id,
        dest.to_string_lossy()
    );

    StatusCode::CREATED
}
