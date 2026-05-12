
# Super Type States / Advanced Typestate Pattern

Let's cover how to implement the **Super Typestate Pattern** in Rust.

## Goal

Our goal is to build a **User Session** system where the struct **physically changes its memory layout and behavior** at compile time depending on its state.

In the end we should be able to do this:

```rust
fn main() {
    let session = Session::<Anonymous>::new(1);
    session.print_status();
    // [Anonymous] Session 1 - token field: 0 bytes, meta field: 0 bytes

    let session = session.login("eyJhbGciOi...".to_string(), User { id: 42, name: "Alice".into() });
    session.print_status();
    // [Authenticated] Session 1 - token field: 24 bytes, meta field: 32 bytes

    let session = session.logout();
    session.print_status();
    // [Terminated] Session 1 - token field: 0 bytes, meta field: 0 bytes
}
```

Notice how the **memory size** of the `token` and `meta` fields **morphs** between 0 bytes and real data depending on the state? That is the Super Typestate Pattern.

> [!IMPORTANT]
> If you haven't read the [State Machines / Typestate Pattern](../state-machines/Readme.md) chapter yet, I recommend doing that first!
> This chapter builds on top of the basic Typestate Pattern and takes it much further.

## Why do we need this?

Imagine you are building a **web application with user sessions**. A session goes through three stages:

1. **Anonymous** - the user just opened the app. No auth token, no user profile.
2. **Authenticated** - the user logged in. Now you have a real token and a user profile.
3. **Terminated** - the session ended. Token and user data are gone.

### The conventional way

You would probably use `Option<T>` for the token and user data:

```rust
pub struct Session {
    pub id: u64,
    pub token: Option<String>,
    pub meta: Option<User>,
}
```

But this comes with problems:

- You pay the **memory overhead** of the `Option` tag for *every* session, even when the fields are `None`.
- You have to write **`.unwrap()`** or **`if let Some(...)`** everywhere to access the token and user data.
  Forget one check? Runtime panic.
- You have to write **`match`** or **`if/else`** guards in methods to check the session state at runtime.
  Forget one check? Bug.

### What the Super Typestate Pattern does instead

It uses **Associated Types** inside a trait to let each state decide the *concrete type* of each field.
When a field isn't needed by the current state, it becomes `()` *(the unit type)* which takes **0 bytes** in memory.

The result:

- **No `Option` overhead** - fields that aren't needed vanish from memory.
- **No `.unwrap()`** - the compiler guarantees that a field exists when the state says it exists.
- **No runtime state checks** - methods only exist for the states where they make sense. Calling the wrong method is a **compile-time error**.

## The mechanic that the Super Typestate Pattern leverages

Before we jump into the implementation, let's understand the core mechanic: **Associated Types in traits**.

Look at this piece of code:

```rust
pub trait Container {
    type Item;
}

pub struct Small;
pub struct Big;

impl Container for Small {
    type Item = ();      // 0 bytes
}

impl Container for Big {
    type Item = String;  // 24 bytes
}

pub struct Box<S: Container> {
    pub content: S::Item,
}
```

When you write `Box<Small>`, the field `content` has type `()` - zero bytes.
When you write `Box<Big>`, the field `content` has type `String` - 24 bytes.

The **same struct** `Box<S>` has a **different physical memory layout** depending on `S`.
The compiler figures out the size at compile time. There is no runtime cost.

```vim
Box<Small>:  content: ()       -> 0 bytes total

Box<Big>:    content: String   -> 24 bytes total
```

This is the foundation of the Super Typestate Pattern.
Now let's use it to build something real.

## Implementation

We are building a session system with three states: `Anonymous`, `Authenticated`, and `Terminated`.

### Step 1: The Blueprint (The Trait)

First we define a trait that acts as a **contract** for every state in our system:

```rust
// src/session.rs

pub trait SessionState {
    type AuthToken;
    type UserMeta;

    fn status_label() -> &'static str;
}
```

This trait says: *"Any state used in this system must declare what an `AuthToken` looks like, what a `UserMeta` looks like, and provide a label for logging."*
The `type AuthToken;` and `type UserMeta;` lines are **Associated Types** - the concrete type is decided by each state implementation.

### Step 2: The Zero-Cost States

Now we define our three states as **Zero-Sized Types** *(marker structs with no fields)*:

```rust
// src/session.rs

// -- snip --

pub struct Anonymous;
pub struct Authenticated;
pub struct Terminated;
```

These structs have **no fields** - they exist purely as compile-time markers.
Now we implement the `SessionState` trait for each one:

```rust
// src/session.rs

// -- snip --

impl SessionState for Anonymous {
    type AuthToken = ();   // no token -> 0 bytes
    type UserMeta = ();    // no user data -> 0 bytes

    fn status_label() -> &'static str { "Anonymous" }
}

impl SessionState for Authenticated {
    type AuthToken = String;   // JWT / bearer token
    type UserMeta = User;      // full user record

    fn status_label() -> &'static str { "Authenticated" }
}

impl SessionState for Terminated {
    type AuthToken = ();   // token discarded -> 0 bytes
    type UserMeta = ();    // user data discarded -> 0 bytes

    fn status_label() -> &'static str { "Terminated" }
}
```

Look at what just happened:

- In the `Anonymous` state, both `AuthToken` and `UserMeta` are `()` - **0 bytes** each.
- In the `Authenticated` state, `AuthToken` is a `String` and `UserMeta` is a `User` struct - **real data**.
- In the `Terminated` state, both fields go back to `()` - **0 bytes** again.

The `status_label()` function is also state-dependent. We will see later how the compiler wires up the correct function at compile time with no `if/else` needed.

Let's also define the `User` struct that `Authenticated` uses:

```rust
// src/session.rs

// -- snip --

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
}
```

### Step 3: The Morphing Struct

Now we define the `Session` struct with fields that use the **Associated Types** from the trait:

```rust
// src/session.rs

// -- snip --

pub struct Session<S: SessionState> {
    pub id: u64,                // always present (8 bytes)
    pub token: S::AuthToken,    // () or String depending on state
    pub meta: S::UserMeta,      // () or User depending on state
}
```

The `id` field is a plain `u64` - it always exists.
But `token` and `meta` **change their type** depending on `S`:

```vim
Session<Anonymous>:    id: u64, token: (),     meta: ()       -> ~8 bytes

Session<Authenticated>: id: u64, token: String,  meta: User     -> ~64 bytes

Session<Terminated>:   id: u64, token: (),     meta: ()       -> ~8 bytes
```

Compare this with the `Option` approach:

```vim
Option-based Session:   id: u64, token: Option<String>, meta: Option<User>  -> ~80+ bytes ALWAYS

Super Typestate:        ~8 bytes when anonymous, ~64 bytes when authenticated
```

With `Option`, you pay for the `Some`/`None` tag AND for the space reserved for the inner type, even when the value is `None`. The Super Typestate Pattern avoids that.

### Step 4: Common Methods (Available on ALL states)

We implement methods that work on **any** state by using a generic `impl` block:

```rust
// src/session.rs

// -- snip --

impl<S: SessionState> Session<S> {
    pub fn print_status(&self) {
        println!(
            "[{}] Session {} - token field: {} bytes, meta field: {} bytes",
            S::status_label(),
            self.id,
            std::mem::size_of::<S::AuthToken>(),
            std::mem::size_of::<S::UserMeta>(),
        );
    }

    pub fn memory_overview(&self) {
        println!("  sizeof id   : {} bytes", std::mem::size_of::<u64>());
        println!("  sizeof token: {} bytes", std::mem::size_of::<S::AuthToken>());
        println!("  sizeof meta : {} bytes", std::mem::size_of::<S::UserMeta>());
        println!(
            "  total       : ~{} bytes (approx, excluding alignment padding)",
            std::mem::size_of::<u64>()
                + std::mem::size_of::<S::AuthToken>()
                + std::mem::size_of::<S::UserMeta>(),
        );
    }
}
```

Notice the `S::status_label()` call. Because `S` is known at compile time, the compiler directly calls the correct `status_label()` function. It runs **just as fast** as if you had hardcoded the string. No virtual dispatch, no `if/else`, zero runtime overhead.

### Step 5: State-Specific Methods

Now we add methods that **only exist in certain states**:

```rust
// src/session.rs

// -- snip --

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

impl Session<Terminated> {
    pub fn renew(self, id: u64) -> Session<Anonymous> {
        println!("[Terminated] Starting a fresh anonymous session");
        Session::new(id)
    }
}
```

Let's break down the important parts:

**The `login()` method** - This is a **state transition**. It consumes `self` *(the `Anonymous` session)* and returns a `Session<Authenticated>`. This is the same concept as in the [basic Typestate Pattern](../state-machines/Readme.md), but now the returned struct has a **completely different memory layout** - the `()` fields got replaced with real data.

**The `token()` and `user()` methods** - These return references to the `String` and `User` fields directly. No `.unwrap()`. The compiler guarantees that `self.token` is a `String` and `self.meta` is a `User` because these methods only exist on `Session<Authenticated>`.

**The `logout()` method** - Another state transition. It consumes the `Session<Authenticated>` and returns a `Session<Terminated>`. The `String` and `User` fields get **automatically dropped** when `self` goes out of scope inside the function. The new struct has `()` fields again - 0 bytes.

Let's put it all together for completeness:

```rust
#![allow(unused)]

use std::mem::size_of;

// 1. THE BLUEPRINT

pub trait SessionState {
    type AuthToken;
    type UserMeta;
    fn status_label() -> &'static str;
}

// 2. THE ZERO-COST STATES

pub struct Anonymous;
pub struct Authenticated;
pub struct Terminated;

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
}

impl SessionState for Anonymous {
    type AuthToken = ();
    type UserMeta = ();
    fn status_label() -> &'static str { "Anonymous" }
}

impl SessionState for Authenticated {
    type AuthToken = String;
    type UserMeta = User;
    fn status_label() -> &'static str { "Authenticated" }
}

impl SessionState for Terminated {
    type AuthToken = ();
    type UserMeta = ();
    fn status_label() -> &'static str { "Terminated" }
}

// 3. THE MORPHING STRUCT

pub struct Session<S: SessionState> {
    pub id: u64,
    pub token: S::AuthToken,
    pub meta: S::UserMeta,
}

// 4. COMMON METHODS

impl<S: SessionState> Session<S> {
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

impl Session<Anonymous> {
    pub fn new(id: u64) -> Self {
        Self { id, token: (), meta: () }
    }

    pub fn login(self, token: String, user: User) -> Session<Authenticated> {
        Session { id: self.id, token, meta: user }
    }
}

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

    pub fn logout(self) -> Session<Terminated> {
        println!(
            "[Authenticated] Logging out user '{}' - dropping token & user data",
            self.meta.name
        );
        Session { id: self.id, token: (), meta: () }
    }
}

impl Session<Terminated> {
    pub fn renew(self, id: u64) -> Session<Anonymous> {
        println!("[Terminated] Starting a fresh anonymous session");
        Session::new(id)
    }
}
```

## Testing everything

Now let's use our `Session` in `src/main.rs` and see the morphing in action:

```rust
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
        User { id: 42, name: "Alice".to_string() },
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
```

Running this prints:

```bash
$ cargo run
[Anonymous] Session 1 - token field: 0 bytes, meta field: 0 bytes
  sizeof id   : 8 bytes
  sizeof token: 0 bytes
  sizeof meta : 0 bytes
  total       : ~8 bytes (approx, excluding alignment padding)

[Authenticated] Session 1 - token field: 24 bytes, meta field: 32 bytes
  sizeof id   : 8 bytes
  sizeof token: 24 bytes
  sizeof meta : 32 bytes
  total       : ~64 bytes (approx, excluding alignment padding)

  token: eyJhbGciOi...
  user:  Alice (id 42)
[Authenticated] Refreshing token for user 'Alice' (id 42)

[Authenticated] Logging out user 'Alice' - dropping token & user data
[Terminated] Session 1 - token field: 0 bytes, meta field: 0 bytes
  sizeof id   : 8 bytes
  sizeof token: 0 bytes
  sizeof meta : 0 bytes
  total       : ~8 bytes (approx, excluding alignment padding)

[Terminated] Starting a fresh anonymous session
[Anonymous] Session 2 - token field: 0 bytes, meta field: 0 bytes
  sizeof id   : 8 bytes
  sizeof token: 0 bytes
  sizeof meta : 0 bytes
  total       : ~8 bytes (approx, excluding alignment padding)
```

Look at the **memory numbers**. When the session is `Anonymous` or `Terminated`, the token and user meta fields take 0 bytes. When it is `Authenticated`, they expand to their real sizes. That is the Super Typestate Pattern.

Now let's misuse the API and see what happens:

```rust
mod session;
use session::{Anonymous, Session, User};

fn main() {
    let session = Session::<Anonymous>::new(1);
    session.token();   // ERROR! token() only exists on Session<Authenticated>
}
```

This causes a compile-time error:

```bash
error[E0599]: no method named `token` found for struct `Session<Anonymous>` in the current scope
 --> src/main.rs:5:13
  |
5 |     session.token();
  |             ^^^^^-- help: remove the arguments
  |             |
  |             field, not a method
  |
  ::: src/session.rs:70:1
   |
70 | pub struct Session<S: SessionState> {
   | ----------------------------------- method `token` not found for this struct

For more information about this error, try `rustc --explain E0599`.
```

Let's try another misuse - calling `login()` on an already authenticated session:

```rust
mod session;
use session::{Authenticated, Session, User};

fn main() {
    let session = Session::<Authenticated> { /* ... */ };
    session.login("token".into(), User { id: 1, name: "Bob".into() });
}
```

This also causes a compile-time error:

```bash
error[E0599]: no method named `login` found for struct `Session<Authenticated>` in the current scope
 --> src/main.rs:11:13
  |
11 |     session.login("token".into(), User { id: 1, name: "Bob".into() });
  |             ^^^^^ method not found in `Session<Authenticated>`
  |
  ::: src/session.rs:70:1
   |
70 | pub struct Session<S: SessionState> {
   | ----------------------------------- method `login` not found for this struct
   |
   = note: the method was found for
           - `Session<Anonymous>`

For more information about this error, try `rustc --explain E0599`.
```

As you can see, if the user of our `Session` lib misuses it, there are compile-time errors! The compiler catches it for you.

## The big picture: Typestate vs Super Typestate

You might wonder: *"How is this different from the basic Typestate Pattern I already learned?"*

The basic Typestate Pattern *(from the [State Machines chapter](../state-machines/Readme.md))* uses `PhantomData<State>` to track state and `Option<T>` for fields that may or may not exist:

```rust
// Basic Typestate Pattern
pub struct TcpConnection<State> {
    stream: Option<TcpStream>,   // always occupies memory for Option tag
    state: PhantomData<State>,   // 0 bytes, just a marker
}
```

The `stream` field is always an `Option<TcpStream>`, regardless of the state. You still need `.unwrap()` to access it.

The **Super Typestate Pattern** uses **Associated Types** to let the struct's fields *themselves* change type:

```rust
// Super Typestate Pattern
pub struct Session<S: SessionState> {
    pub id: u64,
    pub token: S::AuthToken,    // () or String, decided by the state
    pub meta: S::UserMeta,      // () or User, decided by the state
}
```

| Feature | Basic Typestate | Super Typestate |
|---|---|---|
| Restrict methods by state | Yes | Yes |
| Morph data fields | No, uses `Option<T>` | Yes, Associated Types |
| `.unwrap()` needed | Yes | No |
| Memory overhead when field absent | `Option` tag *(usually 1 byte)* | 0 bytes `()` type |
| Morph behavior *(state-specific functions)* | No, uses `if/else` or `match` | Yes, trait methods |

Both patterns are useful. Start with the basic Typestate Pattern when you only need to restrict method calls by state. Reach for the Super Typestate Pattern when you also want to **optimize memory** and **eliminate runtime checks**.

## Summary

We implemented a `Session` struct whose fields **physically morph between 0 bytes and real data** depending on its state, all verified at compile time.

- Fields that aren't needed become `()` and take **0 bytes** - no `Option` overhead.
- Methods that only make sense in a certain state only exist in that state - calling the wrong method is a **compile-time error**.
- The trait's associated functions *(like `status_label()`)* act as a **zero-cost strategy pattern** - the compiler wires up the correct function at compile time.

**Congrats!**
