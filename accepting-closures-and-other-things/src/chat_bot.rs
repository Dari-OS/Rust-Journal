#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Event {
    Message,
    UserBan,
    UserJoin,
    UserLeave,
    Metnion,
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
