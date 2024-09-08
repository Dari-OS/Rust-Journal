# Rust Journal

This journal has a growing collection of magic code and patterns that I found during my journey of mastering Rust

> [!NOTE]
> This project is inspired by the GitHub repo [Rust Magic Pattern](https://github.com/alexpusch/rust-magic-patterns/).  
> I recommend checking it out!

## [Bitflags](bitflags/Readme.md)

Sometimes it gets very tedious to get **muliple Flags** as paramaters in a function/method.  
But Bitflags help to handle that scenario in a very simple (from the user perspective) and elegant way.

> _Bitflags_::<ins>**Are**</ins> | _Bitflags_::<ins>**You**</ins> | _Bitflags_::<ins>**Interested**</ins>

## [State Maschines / (Typestate Pattern)](state-machines/Readme.md)

One of **my favourite coding patters** are **State Machines**.  
But not just any State Machine.
You can implement the State Machine pattern in Rust in such away to have **compile time checks at state transitions**.

> Sounds **interesting** huh?

## [Accepting closure and other thing at the same time!](accepting-closures-and-other-things/Readme.md)

> You may wonder if this is possible?

I found an **amazing design pattern** in the populare crate **[Notify](https://github.com/notify-rs/notify/)**. It allows a function to accept a closure and a Channel Sender _(just an example)_ at the same time!
