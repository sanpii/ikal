/**
 * See [3.6. Calendar Components](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6)
 */
mod vcalendar;
mod vevent;
mod vfreebusy;
mod vjournal;
mod vtodo;

pub mod valarm;
pub mod vtimezone;

pub use valarm::VAlarm;
pub use vcalendar::*;
pub use vevent::*;
pub use vfreebusy::*;
pub use vjournal::*;
pub use vtimezone::VTimezone;
pub use vtodo::*;

#[derive(Debug, PartialEq)]
pub(crate) enum Component {
    Alarm(crate::VAlarm),
    Event(crate::VEvent),
    Freebusy(crate::VFreebusy),
    Journal(crate::VJournal),
    Timezone(crate::VTimezone),
    Todo(crate::VTodo),
}
