// ======================================================================
// 📍 FILE: elysia_core/src/http/install.rs
// 📝 ROLE:
//   Installeert een reeds geüploade module-zip
//   - Admin-only
//   - Extract naar plugins/
//   - Geen execution
// ======================================================================

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
};
use std::{fs, path::PathBuf};
use zip::ZipArchive;

pub async fn install_module(
    Path(upload_id): Path<String>,
) -> impl IntoResponse {
    let zip_path = PathBuf::from("elysia_data/uploads/modules")
        .join(format!("{}.zip", upload_id));

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

    // plugin-id = zipnaam (of later uit manifest)
    let plugin_id = upload_id.to_lowercase();
    let dest = PathBuf::from("plugins").join(&plugin_id);

    if dest.exists() {
        return StatusCode::CONFLICT;
    }

    if fs::create_dir_all(&dest).is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).unwrap();

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

            std::io::copy(&mut entry, &mut outfile).ok();
        }
    }

    StatusCode::CREATED
}
