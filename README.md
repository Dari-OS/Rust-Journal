# Rust Journal

This journal has a growing collection of magic code and patterns that I found during my journey of mastering Rust

> [!NOTE]
> This project is inspired by the GitHub repo [Rust Magic Pattern](https://github.com/alexpusch/rust-magic-patterns/).  
> I recommend checking it out!

## [Bitflags](Bitflags/Readme.md)
Sometimes it gets very tedious to get __muliple Flags__ as paramaters in a function/method.  
But Bitflags help to handle that scenario in a very simple (from the user perspective) and elegant way.
> *Bitflags*::<ins>__Are__</ins> | *Bitflags*::<ins>__You__</ins> | *Bitflags*::<ins>__Interested__</ins>


## [State Maschines / (Typestate Pattern)](state-machines/Readme.md)
One of __my favourite coding patters__ are __State Machines__.  
But not just any State Machine. 
You can implement the State Machine pattern in Rust in such away to have __compile time checks at state transitions__.  
> Sounds __interesting__ huh?

## [Accepting closure and other thing at the same time!](accepting-closures-and-other-things/Readme.md)  
> You may wonder if this is possible?

I found an __amazing design pattern__ in the populare crate __[Notify](https://github.com/notify-rs/notify/)__.  It allows a function to accept a closure and a Channel Sender *(just an example)* at the same time!  
