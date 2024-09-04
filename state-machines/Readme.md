
# State Machines / Typestate Pattern 

State Machines are a clean API design pattern to code APIs, Builders, and even more!  
The goal of State Machines are to indicate a misuse of the API during the compile-time.

## Use cases

State Machines are usefull for building clean APIs and *(`struct`)* Builders.
They make use of (for example) a lib more intuitive and easier for the *end-dev*.

## Scenario

We are building a `TcpStream` that allows the user to create a __TCP__ connection to a __remote/local server__. The connection doesn't have to be initialized with a set host. And after closing the connection you shouldn't be able to send/receive any further data.

## The conventional way

You may think of using an __`Enum`__ internally and use a `match` expression in each method. This gets tedious very quickly. You may even think of another method of doing this, but let me intreduce you to __STATE MACHINES__! 

## The mechanic that State Machines leverage

Look at this piece of code:

```rust
pub struct Foo<T> {
    bar: T,
}

impl Foo<String> {
    pub fn string_function(&self) {
        println!("I am a string");
    }
}

impl Foo<&str> {
    pub fn str_function(&self) {
        println!("I am a string SLICE")
    }
}

fn main() {
    let foo_string = Foo { bar: String::new() };

    let foo_str_slice = Foo { bar: "" };

    foo_string.string_function();
    //OUTPUT: I am a string

    foo_str_slice.str_function();
    //OUTPUT: I am a string SLICE
}


```

See? You are having different methods depending on the used datatype _(in bar)_.
If you try to use the methods of a diffrent datatype you'll get a compile time error.

```rust
// --snip--

fn main() {
    let foo_string = Foo { bar: String::new() };

    foo_string.str_function();
}
```

This would cause a compiler error:

```bash
error[E0599]: no method named `str_function` found for struct `Foo<String>` in the current scope
  --> src/main.rs:22:16
   |
1  | pub struct Foo<T> {
   | ----------------- method `str_function` not found for this struct
...
22 |     foo_string.str_function();
   |                ^^^^^^^^^^^^
   |
```

__State machines__ use exactly this concept by using an internal state that gets only changed from the inside. Let's begin be declaring states:

```rust
pub struct Connected;
pub struct Disconnected;
pub struct NotConnected;
```

Now we will define the basic struct for the `TcpConnection`.   
*(We are using the TcpStream internally, because we don't want to implement a full tcp stream for ourselves just for this example)*

```rust
pub struct TcpConnection {
    stream: Option<TcpStream>,
}

```

Now we will have to add the states to be able to have different methods for each states.

```rust
use std::marker::PhantomData;

// states

pub struct TcpConnection<State = NotConnected> {
    stream: Option<TcpStream>,
    state: PhantomData<State>,
}

```

You may be wondering what this new type __(`PhantomData`)__ is and why we set a default state.

- `PhantomData` 
    - The __Compiler__ may optimize this, because `PhantomData` indicates that the Type wont get used.
    - `PhantomData<State>` allows State to be any type, even those you can't instantiate, which can be useful in more complex generic code.
- `TcpConnection<State = NotConnected>`
    - Avoids explicitly specifing the state type when creating a new connection *(`TcpConnection::<NotConnected>::new()`)*
    - allows the compiler to catch invalid state transitions at compile time. By defaulting to `NotConnecte`, you ensure that the user is forced to follow the correct sequence of operations

Adding the different implementaion for each `State`:

```rust
// -- snip --

impl TcpConnection<NotConnected> {
    // todo!();
}

impl TcpConnection<Connected> {
    // todo!();
}

impl TcpConnection<Disconnected> {
    // todo!():
}

```

#### A State Transition

A State Transition is __transitioning__ from one state to another.  
We are achieving this by creating a new `TcpConnection` with the same data, BUT with the new state.

```rust
 pub fn state_transtion(self) -> TcpConnection<Disconnected> {
        TcpConnection {
            stream: None,
            state: PhantomData,
        }
    }
```

`Disconnected` is the desired state in this case.  
We'll implement all state transitions:

```rust
// -- snip --

impl TcpConnection<NotConnected> {

    pub fn set_host<T: ToSocketAddrs>(
        self,
        host: T,
    ) -> Result<TcpConnection<Connected>, io::Error> {
        Ok(TcpConnection {
            stream: Some(TcpStream::connect(host)?),
            state: PhantomData::<Connected>,
        })
    }

    pub fn connect<T: ToSocketAddrs>(host: T) -> Result<TcpConnection<Connected>, io::Error> {
        Ok(TcpConnection {
            stream: Some(TcpStream::connect(host)?),
            state: PhantomData::<Connected>,
        })
    }
}

impl TcpConnection<Connected> {
    pub fn close(self) -> TcpConnection<Disconnected> {
        if self.stream.is_some() {
            drop(self.stream.unwrap());
        }
        TcpConnection {
            stream: None,
            state: PhantomData,
        }
    }
}

impl TcpConnection<Disconnected> {
    pub fn reconnect<T: ToSocketAddrs>(
        self,
        host: T,
    ) -> Result<TcpConnection<Connected>, io::Error> {
        Ok(TcpConnection {
            stream: Some(TcpStream::connect(host)?),
            state: PhantomData::<Connected>,
        })
    }
}
```

Let's add the rest of the code to make the `TcpConnection` functional:  

```rust 
#![allow(unused)]

use std::{
    io::{self, Read, Write},
    marker::PhantomData,
    net::{TcpStream, ToSocketAddrs},
};

pub struct Connected;
pub struct Disconnected;
pub struct NotConnected;

pub struct TcpConnection<State = NotConnected> {
    stream: Option<TcpStream>,
    state: PhantomData<State>,
}

impl TcpConnection<NotConnected> {
    pub fn new() -> Self {
        Self {
            stream: None,
            state: PhantomData,
        }
    }

    pub fn set_host<T: ToSocketAddrs>(
        self,
        host: T,
    ) -> Result<TcpConnection<Connected>, io::Error> {
        Ok(TcpConnection {
            stream: Some(TcpStream::connect(host)?),
            state: PhantomData::<Connected>,
        })
    }

    pub fn connect<T: ToSocketAddrs>(host: T) -> Result<TcpConnection<Connected>, io::Error> {
        Ok(TcpConnection {
            stream: Some(TcpStream::connect(host)?),
            state: PhantomData::<Connected>,
        })
    }
}

impl TcpConnection<Connected> {
    pub fn send(&mut self, data_to_send: &[u8]) -> io::Result<usize> {
        self.stream.as_mut().unwrap().write(data_to_send)
    }

    pub fn receive(&mut self, data_to_read: &mut [u8]) -> io::Result<usize> {
        self.stream.as_mut().unwrap().read(data_to_read)
    }

    pub fn close(self) -> TcpConnection<Disconnected> {
        if self.stream.is_some() {
            drop(self.stream.unwrap());
        }
        TcpConnection {
            stream: None,
            state: PhantomData,
        }
    }
}

impl TcpConnection<Disconnected> {
    pub fn reconnect<T: ToSocketAddrs>(
        self,
        host: T,
    ) -> Result<TcpConnection<Connected>, io::Error> {
        Ok(TcpConnection {
            stream: Some(TcpStream::connect(host)?),
            state: PhantomData::<Connected>,
        })
    }
}
```

## Testing everything

To test the `TcpConnection` we have to run a server. We can run a server (for testing purpose). We have an example echo server under [`example_server.rs`](src/example_server.rs). Let's run the example server.

```bash
$ cargo run -q --bin server
The server is up and running on port 8080!
```

Underneath there is an example on how to use the `TcpConnection`:

```rust
mod tcp_connector;
use tcp_connector::TcpConnection;
fn main() {
    let connection = TcpConnection::new();
    let mut connection = connection.set_host("127.0.0.1:8080").unwrap();

    let _ = connection.send("Hello Rust!".as_bytes());
    let mut received = [0u8; 1024];

    let read = connection.receive(&mut received);

    println!(
        "{}",
        String::from_utf8(received[..read.unwrap()].to_vec()).unwrap()
    );

    connection.close();
}
```


This is what we see from the `TcpConnection` side:

```bash
$ cargo run -q --bin example
Hello Rust!
```

And this gets printed on the Server side:

```bash
$ cargo run -q --bin server
The server is up and running on port 8080!
A client connected with the address: 127.0.0.1:53766
```

Now we miss use our `TcpConnection` api and see what happens:

```rust
mod tcp_connector;
use tcp_connector::TcpConnection;
fn main() {
    let mut connection = TcpConnection::new();
    let _ = connection.send("Hello World".as_bytes());
}
```

This prints this compilation error:

```bash
error[E0599]: no method named `send` found for struct `TcpConnection` in the current scope
  --> src/main.rs:5:24
   |
5  |     let _ = connection.send("Hello World".as_bytes());
   |                        ^^^^ method not found in `TcpConnection`
   |
  ::: src/tcp_connector.rs:13:1
   |
13 | pub struct TcpConnection<State = NotConnected> {
   | ---------------------------------------------- method `send` not found for this struct
   |
   = note: the method was found for
           - `TcpConnection<Connected>`

For more information about this error, try `rustc --explain E0599`.
```

Let's miss use the API again.

```rust
mod tcp_connector;
use tcp_connector::TcpConnection;
fn main() {
    let mut connection = TcpConnection::connect("127.0.0.1:8080").unwrap();
    let _ = connection.set_host("127.0.0.1:8080");
}

```

Here is the error message:

```bash
error[E0599]: no method named `set_host` found for struct `TcpConnection<Connected>` in the current scope
  --> src/main.rs:5:24
   |
5  |     let _ = connection.set_host("127.0.0.1:8080");
   |                        ^^^^^^^^ method not found in `TcpConnection<Connected>`
   |
  ::: src/tcp_connector.rs:13:1
   |
13 | pub struct TcpConnection<State = NotConnected> {
   | ---------------------------------------------- method `set_host` not found for this struct
   |
   = note: the method was found for
           - `TcpConnection`

For more information about this error, try `rustc --explain E0599`.
```

As you can see, if the user of our `TcpConnection` lib misuses it, there are compile itme errors! That makes our lib way easier to use correctly! 

## Finished!

everything worked perfectly fine!  
I hope you understand the technic behinde State Machines
