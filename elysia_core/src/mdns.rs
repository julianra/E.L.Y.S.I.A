// ======================================================================
// 📍 FILE: elysia/elysia_core/src/mdns.rs
// ======================================================================

use mdns_sd::{ServiceDaemon, ServiceInfo};

pub fn start_mdns(port: u16) -> Result<ServiceDaemon, Box<dyn std::error::Error + Send + Sync>> {
    // Local IP ophalen (zoals je al had)
    let local_ip = local_ip_address::local_ip()?.to_string();
    log::info!("[CORE] Using local IP for mDNS: {}", local_ip);

    let mdns = ServiceDaemon::new()?;

    let service_type = "_elysia._tcp.local.";
    let instance_name = "Elysia Node";

    // HOSTNAME = IP (Android kan deze resolven)
    let host_name = &local_ip;

    let properties = [("path", "/")];

    let service_info = ServiceInfo::new(
        service_type,
        instance_name,
        host_name,     // <-- IP in plaats van "elysia.local"
        &local_ip[..], // <-- Adapter IP
        port,
        &properties[..],
    )?;

    mdns.register(service_info)?;

    log::info!(
        "[CORE] mDNS active: {} on {} (port {})",
        instance_name, service_type, port
    );

    Ok(mdns)
}
