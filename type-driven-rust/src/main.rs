mod session;

use session::{Anonymous, Session, User};

fn main() {
    // Step 1: Anonymous session
    // No token, no user data. Both fields are () (0 bytes).
    let session = Session::<Anonymous>::new(1);
    session.print_status();
    session.memory_overview();

    println!();

    // Step 2: Login -> morph into Authenticated
    // The () fields get replaced with a real String token and User.
    let mut session = session.login(
        "eyJhbGciOi...".to_string(),
        User {
            id: 42,
            name: "Alice".to_string(),
        },
    );
    session.print_status();
    session.memory_overview();

    println!();

    // Access token and user directly, no unwrap!
    println!("  token: {}", session.token());
    println!("  user:  {} (id {})", session.user().name, session.user().id);

    // refresh_token is only available in Authenticated state
    session.refresh_token("eyJnewToken...".to_string());

    println!();

    // Step 3: Logout -> morph into Terminated
    // String and User get dropped, fields go back to ().
    let session = session.logout();
    session.print_status();
    session.memory_overview();

    println!();

    // Step 4: Renew -> back to Anonymous
    let session = session.renew(2);
    session.print_status();
    session.memory_overview();
}
