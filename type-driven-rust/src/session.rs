use std::mem::size_of;

// 1. THE BLUEPRINT
// A trait with Associated Types. Each state decides what AuthToken and UserMeta
// look like. When a field isn't needed, it becomes () (0 bytes).

pub trait SessionState {
    type AuthToken;
    type UserMeta;

    fn status_label() -> &'static str;
}

// 2. THE ZERO-COST STATES
// Marker structs with no fields (Zero-Sized Types).
// The compiler erases them at runtime.

pub struct Anonymous;
pub struct Authenticated;
pub struct Terminated;

impl SessionState for Anonymous {
    type AuthToken = ();   // no token -> 0 bytes
    type UserMeta = ();    // no user data -> 0 bytes

    fn status_label() -> &'static str {
        "Anonymous"
    }
}

impl SessionState for Authenticated {
    type AuthToken = String;   // JWT / bearer token
    type UserMeta = User;      // full user record

    fn status_label() -> &'static str {
        "Authenticated"
    }
}

impl SessionState for Terminated {
    type AuthToken = ();   // token discarded -> 0 bytes
    type UserMeta = ();    // user data discarded -> 0 bytes

    fn status_label() -> &'static str {
        "Terminated"
    }
}

// 3. THE MORPHING STRUCT
// token and meta change type depending on the state.
// In Anonymous state those two fields take 0 bytes each.

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
}

pub struct Session<S: SessionState> {
    pub id: u64,                // always present
    pub token: S::AuthToken,    // () or String depending on state
    pub meta: S::UserMeta,      // () or User depending on state
}

// 4. COMMON METHODS (available on ANY state)

impl<S: SessionState> Session<S> {
    pub fn session_id(&self) -> u64 {
        self.id
    }

    pub fn print_status(&self) {
        println!(
            "[{}] Session {} - token field: {} bytes, meta field: {} bytes",
            S::status_label(),
            self.id,
            size_of::<S::AuthToken>(),
            size_of::<S::UserMeta>(),
        );
    }

    pub fn memory_overview(&self) {
        println!("  sizeof id   : {} bytes", size_of::<u64>());
        println!("  sizeof token: {} bytes", size_of::<S::AuthToken>());
        println!("  sizeof meta : {} bytes", size_of::<S::UserMeta>());
        println!(
            "  total       : ~{} bytes (approx, excluding alignment padding)",
            size_of::<u64>() + size_of::<S::AuthToken>() + size_of::<S::UserMeta>(),
        );
    }
}

// 5. STATE-SPECIFIC METHODS

// Methods that ONLY exist when the session is Anonymous.
impl Session<Anonymous> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            token: (),
            meta: (),
        }
    }

    // Consumes the anonymous session and returns an authenticated one.
    // The () fields get replaced with real data. No Option, no unwrap.
    pub fn login(self, token: String, user: User) -> Session<Authenticated> {
        Session {
            id: self.id,
            token,
            meta: user,
        }
    }
}

// Methods that ONLY exist when the session is Authenticated.
impl Session<Authenticated> {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn user(&self) -> &User {
        &self.meta
    }

    pub fn refresh_token(&mut self, new_token: String) {
        println!(
            "[Authenticated] Refreshing token for user '{}' (id {})",
            self.meta.name, self.meta.id
        );
        self.token = new_token;
    }

    // Consumes the authenticated session and returns a terminated one.
    // String and User get dropped, fields go back to ().
    pub fn logout(self) -> Session<Terminated> {
        println!(
            "[Authenticated] Logging out user '{}' - dropping token & user data",
            self.meta.name
        );
        Session {
            id: self.id,
            token: (),
            meta: (),
        }
    }
}

// Methods that ONLY exist when the session is Terminated.
impl Session<Terminated> {
    pub fn renew(self, id: u64) -> Session<Anonymous> {
        println!("[Terminated] Starting a fresh anonymous session");
        Session::new(id)
    }
}
