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
