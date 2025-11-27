// ======================================================================
// 📍 FILE: elysia/elysia_core/src/mdns.rs
//
// 📝 BESCHRIJVING:
//   mDNS helper voor ELYSIA.
//   - Start een mDNS-daemon
//   - Registreert een Elysia-service op het lokale netwerk
//
// 🔧 TAKEN:
//   - Zorgt dat Orbit en andere clients ELYSIA automatisch kunnen vinden
// ======================================================================

use mdns_sd::{ServiceDaemon, ServiceInfo};

pub fn start_mdns(port: u16) -> Result<ServiceDaemon, Box<dyn std::error::Error + Send + Sync>> {
    // 1. Daemon starten
    let mdns = ServiceDaemon::new()?;

    // 2. Verplichte velden
    let service_type = "_elysia._tcp.local.";
    let instance_name = "Elysia Node";
    let host_name = "elysia.local."; // hostnaam voor mDNS

    // 3. IP als string → implementeert AsIpAddrs
    // "0.0.0.0" = OS kiest de juiste interface / IP
    let ip = "0.0.0.0";

    // 4. TXT properties als key–value paren
    let properties = [("path", "/")];

    // 5. Correcte signature voor mdns-sd 0.7.x:
    //    new(service_type, instance_name, host_name, ip, port, properties)
    let service_info = ServiceInfo::new(
        service_type,
        instance_name,
        host_name,
        ip,
        port,
        &properties[..],
    )?;

    // 6. Registreren bij de daemon
    mdns.register(service_info)?;

    log::info!(
        "[CORE] mDNS active: {} on {} (port {})",
        instance_name,
        service_type,
        port
    );

    Ok(mdns)
}
