# Rust Journal

This journal has a growing collection of magic code that is written in Rust

> [!NOTE]
> This project is inspired by the GitHub repo [Rust Magic Pattern](https://github.com/alexpusch/rust-magic-patterns/).  
> I recommend checking it out!

## [State Maschines](state-machines/Readme.md)
One of my favourite coding patters are State Machines.  
But not just any State Machine. 
You can implement the State Machine pattern in Rust in such away to have compile time checks at state transitions.  
> Sounds __interesting__ huh?

## [Accepting closure and other thing at the same time!](accepting-closures-and-other-things/Readme.md)  
> You may wonder if this is possible?

I found an __amazing design pattern__ in the populare crate __[Notify](https://github.com/notify-rs/notify/)__.  It allows a function to accept a closure and a Channel Sender *(just an example)* at the same time!  
