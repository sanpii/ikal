/**
 * See [3.6. Calendar Components](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6)
 */
mod vevent;
mod vtodo;

pub use vevent::*;
pub use vtodo::*;

#[derive(Debug, PartialEq)]
pub enum Content {
    Event(crate::VEvent),
    Todo(crate::VTodo),
}

impl Default for Content {
    fn default() -> Self {
        Self::Event(crate::VEvent::default())
    }
}

macro_rules! get {
    ($name:ident => $ty:ty) => {
        pub fn $name(&self) -> &$ty {
            match self {
                Self::Event(event) => &event.$name,
                Self::Todo(todo) => &todo.$name,
            }
        }

    }
}

impl Content {
    get!(dtstamp => crate::DateTime);
    get!(uid => String);
    get!(summary => Option<String>);

    pub fn into_event(self) -> crate::VEvent {
        match self {
            Self::Event(event) => event,
            _ => panic!(),
        }
    }

    pub fn into_todo(self) -> crate::VTodo {
        match self {
            Self::Todo(todo) => todo,
            _ => panic!(),
        }
    }
}
