// ======================================================================
// 📍 FILE: elysia/elysia_core/src/router.rs
//
// 📝 BESCHRIJVING:
//   Een eenvoudige router die modules laat API-endpoints registreren.
//   Wordt later vervangen door Axum/Actix, maar structuur blijft.
//
// 🔧 TAKEN:
//   - Opslag van HTTP-routes
//   - Kernel kan geregistreerde routes tonen
//   - Modules kunnen met `add_route` nieuwe endpoints toevoegen
// ======================================================================

#[derive(Default)]
pub struct Router {
    pub routes: Vec<String>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: vec![] }
    }

    pub fn add_route(&mut self, method: &str, path: &str) {
        self.routes.push(format!("{} {}", method, path));
    }
}
