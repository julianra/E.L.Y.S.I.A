// ======================================================================
// 📍 FILE: elysia_core/src/security/auth/subject.rs
// 📝 Represents an authenticated entity inside the Kernel's IPC system.
// ======================================================================

#[derive(Clone)]
pub enum AuthSubject {
    User(String),
    Device(String),
}

impl AuthSubject {
    pub fn username(&self) -> Option<&str> {
        match self {
            AuthSubject::User(u) => Some(u),
            _ => None,
        }
    }

    pub fn device_id(&self) -> Option<&str> {
        match self {
            AuthSubject::Device(id) => Some(id),
            _ => None,
        }
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, AuthSubject::User(u) if u == "admin")
    }
}
