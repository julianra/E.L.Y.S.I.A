// ======================================================================
// 📍 FILE: elysia_core/src/mdns.rs
//
// 📝 BESCHRIJVING:
//   Deze file start de mDNS-service zodat Orbit en andere nodes automatisch
//   de ELYSIA Kernel ontdekken op het lokale netwerk.
//
//   We gebruiken het mDNS service-type:
//       _elysia._tcp.local.
//
//   Elke node broadcast:
//      - unieke node ID
//      - lokale IP
//      - poort (2022)
//      - node capabilities (CPU/RAM/Roles)
//      - versie
//
//   Orbit en CARE/SHIELD nodes gebruiken deze broadcast om te bepalen
//   welke node welke taak uitvoert (Whisper, Safety Engine, Planning).
//
// ======================================================================

use mdns_sd::{ServiceDaemon, ServiceInfo};
use serde_json::json;
use crate::kernel::KernelState;

pub fn start_mdns(port: u16, _state: &KernelState) -> anyhow::Result<ServiceDaemon> {
    let mdns = ServiceDaemon::new()?;

    // -------------------------------------
    // Lokale IP bepalen
    // -------------------------------------
    let local_ip = local_ip_address::local_ip()?
        .to_string();

    // -------------------------------------
    // Unieke node ID genereren / laden
    // -------------------------------------
    let node_id = ensure_node_id();

    // -------------------------------------
    // Node capabilities opstellen
    // -------------------------------------
    let capabilities = json!({
        "id": node_id,
        "type": "core",
        "version": "2.0",
        "ip": local_ip,
        "port": port,
        "roles": [
            "kernel",
            "planner",
            "eventbus",
            "db",
            "router"
        ],
        "cpu": num_cpus::get(),
        "ram": sysinfo_ram_mb(),
    });

    // mDNS properties (max key/value 255 chars)
    let props = [
        ("id", node_id.as_str()),
        ("ip", local_ip.as_str()),
        ("port", &port.to_string()),
        ("capabilities", &capabilities.to_string()),
    ];

    let service_type = "_elysia._tcp.local.";
    let instance_name = "ELYSIA Core Node";

    let service = ServiceInfo::new(
        service_type,
        instance_name,
        &local_ip,
        &local_ip,
        port,
        &props[..],
    )?;

    mdns.register(service)?;

    log::info!(
        "[mDNS] Broadcasting ELYSIA Core → {}:{}",
        local_ip,
        port
    );

    Ok(mdns)
}

// -----------------------------------------------------------
// Unieke node ID persistent opslaan (1x per installatie)
// -----------------------------------------------------------
fn ensure_node_id() -> String {
    let base = dirs::data_local_dir()
        .unwrap()
        .join("elysia");

    std::fs::create_dir_all(&base).ok();

    let file = base.join("node_id");

    if file.exists() {
        return std::fs::read_to_string(&file)
            .unwrap_or_else(|_| "unknown".into());
    }

    // New ID
    let id = uuid::Uuid::new_v4().to_string();
    let _ = std::fs::write(file, &id);
    id
}

// -----------------------------------------------------------
// Systeem RAM ophalen (MB)
// -----------------------------------------------------------
fn sysinfo_ram_mb() -> u64 {
    use sysinfo::System;
    let mut sys = System::new_all();
    sys.refresh_memory();
    sys.total_memory() / 1024
}
