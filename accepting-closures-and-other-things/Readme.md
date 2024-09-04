
# Accepting Closures and Other things

Let's break down the magic behind this.

## Scenario
Let's imagine we are coding a Lib for creating chat bots on a Chat Platform. We want the user to be able to listen to incoming events *(Messages, Mentions, Bans, ...)*.
 
## Desired outcome

We want to allow the user of the lib to pass in a closure to execute the code inside it if an event occurs.
```rust
fn main() {
    chat_bot::listen_events(|event| {
        //TODO: Handle received Events
        todo!();
    });
}

```

We also want to enable our user to pass in something else, for example a channel.

```rust
fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    chat_bot::listen_events(tx);
    loop {
    let event = rx.recv().unwrap();
    // Do something with the received event
    todo!();
    }
}

```

We can take multiple diffrent types that have the same outcome: 

> Sending the events from the Chat Platform to the library user  

## Creating the magic behind this

Let's start by adding some Events to make the example a bit more realistic.

```rust
#[derive(Debug, PartialEq)]
pub enum Event {
    Message,
    UserBan,
    UserJoin,
    UserLeave,
    Mention,
    // Other events
}

```

Now we create the *"first part"* of our `listen_events()` function.

```rust
pub fn listen_events<T>(mut handler: T) {
    todo!();
}
```

We have to introduce some bounds for the Type `T`.  
This is where the magic beginns:  
```rust
pub trait EventHandler {
    fn receive_event(&mut self, event: Event);
}
```

The `EventHandler`is going to be the type that we accept at the `listen_events()` function.  

```rust
pub fn listen_events<T: EventHandler>(mut handler: T) {
    todo!();
}
```

To __accept Channels__ *(To be more precies we have to accept `Sender<Event>`)* we have to `impl` it for the `EventHandler`trait: 

```rust
impl EventHandler for std::sync::mpsc::Sender<Event> {
    fn receive_event(&mut self, event: Event) {
        let _ = self.send(event);
    }
}
```

We are able to pass in channels *(`Sender<Event>`)* to our function.  
But we also want to be able to accept Closures with the `EventHandler` type.  
We do this by implementing the trait `EventHandler` for __everything__ that is the correct type of closure.  

```rust
impl<F> EventHandler for F
where
    F: FnMut(Event) + Send + 'static,
{
    fn receive_event(&mut self, event: Event) {
        self(event)
    }
}

```

The `Send` trait and the `'static` lifetime are not needed, but in a full scale chat bot application it would ensure certain things (Like sending the closure through threads etc).  
That is pretty much it! We have done it! We broke down the magic behind this. Now let's add some code to the body of the `listen_events()` function __for testing purposes__ and move the whole code to a file called `chat_bot.rs`

```rust
// src/chat_bot.rs

#[derive(Debug, PartialEq)]
pub enum Event {
    Message,
    UserBan,
    UserJoin,
    UserLeave,
    Mention,
}

pub trait EventHandler {
    fn receive_event(&mut self, event: Event);
}

impl<F> EventHandler for F
where
    F: FnMut(Event) + Send + 'static,
{
    fn receive_event(&mut self, event: Event) {
        self(event)
    }
}

impl EventHandler for std::sync::mpsc::Sender<Event> {
    fn receive_event(&mut self, event: Event) {
        let _ = self.send(event);
    }
}

pub fn listen_events<T: EventHandler>(mut handler: T) {
    // Some example placeholder code to see if everything works
    handler.receive_event(Event::Message);
}

```

We should see if it works, by adding code that *"tests"* it:

```rust
// src/main.rs

mod chat_bot;

fn main() {
    chat_bot::listen_events(|event| {
        println!("Received the {event:?} event successfully");
        assert_eq!(event, chat_bot::Event::Message)
    });

    let (tx, rx) = std::sync::mpsc::channel();
    chat_bot::listen_events(tx);
    assert_eq!(rx.recv().unwrap(), chat_bot::Event::Message);
}

``` 

This runs successfully, perfect!

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
    Running `target/debug/accepting-closures-and-other-things`
Received the Message event successfully
```

We uncoverd the magic behind this and hopefully you understand it, too!  
You can always try it yourself and come back to this, if you forget about this *"code pattern"*!
