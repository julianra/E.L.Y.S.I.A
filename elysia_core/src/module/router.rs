// ======================================================================
// 📍 FILE: elysia_core/src/module/router.rs
//
// 📝 BESCHRIJVING:
//   Dit bestand definieert helper-functies en structen voor het Advanced
//   Modular Router systeem. Modules leveren gewoon een Axum Router terug,
//   en Kernel monteert dit automatisch onder:
//
//       /api/<module_name>/*
//
//   Zo blijft de Core clean en consistent.
// ======================================================================

use axum::Router;

pub struct ModuleRoute {
    pub name: &'static str,
    pub router: Router,
}

impl ModuleRoute {
    pub fn new(name: &'static str, router: Router) -> Self {
        Self { name, router }
    }
}
