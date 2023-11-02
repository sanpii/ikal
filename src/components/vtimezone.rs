use std::collections::BTreeMap;

#[derive(Debug, Default, PartialEq, crate::Component)]
pub struct VTimezone {
    pub tzid: String,
    pub last_modified: Option<crate::DateTime>,
    pub tzurl: Option<String>,
    #[component(ignore)]
    pub standard: Vec<Prop>,
    #[component(ignore)]
    pub daylight: Vec<Prop>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, String>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, String>,
}

impl VTimezone {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Component {
    Standard(Prop),
    Daylight(Prop),
}

#[derive(Debug, PartialEq, crate::Component)]
pub struct Prop {
    pub dtstart: crate::DateTime,
    pub tzoffsetto: chrono::offset::FixedOffset,
    pub tzoffsetfrom: chrono::offset::FixedOffset,
    pub rrule: Option<crate::Recur>,
    pub comment: Vec<String>,
    #[component(append)]
    pub rdate: Vec<String>,
    pub tzname: Vec<String>,
    #[component(ignore)]
    pub x_prop: BTreeMap<String, String>,
    #[component(ignore)]
    pub iana_prop: BTreeMap<String, String>,
}

impl Default for Prop {
    fn default() -> Self {
        Self::new()
    }
}

impl Prop {
    #[must_use]
    fn new() -> Self {
        Self {
            dtstart: crate::DateTime::default(),
            tzoffsetto: chrono::offset::FixedOffset::east_opt(0).unwrap(),
            tzoffsetfrom: chrono::offset::FixedOffset::east_opt(0).unwrap(),
            rrule: None,
            comment: Vec::new(),
            rdate: Vec::new(),
            tzname: Vec::new(),
            x_prop: BTreeMap::new(),
            iana_prop: BTreeMap::new(),
        }
    }
}