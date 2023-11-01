use std::collections::BTreeMap;

/**
 * See [3.6.1. Event Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.1)
 */
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VEvent {
    pub dtstamp: crate::DateTime,
    pub uid: String,
    pub dtstart: crate::DateTime,
    pub dtend: crate::DateTime,
    pub class: Option<crate::Class>,
    pub created: Option<crate::DateTime>,
    pub description: Option<String>,
    pub geo: Option<crate::Geo>,
    pub last_modified: Option<crate::DateTime>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub priority: Option<u8>,
    pub seq: Option<u32>,
    pub status: Option<crate::Status>,
    pub summary: Option<String>,
    pub transp: Option<crate::TimeTransparency>,
    pub url: Option<String>,
    pub recurid: Option<String>,
    pub rrule: Option<crate::Recur>,
    pub attach: Vec<String>,
    pub attendee: Vec<String>,
    pub categories: Vec<String>,
    pub comment: Vec<String>,
    pub contact: Vec<String>,
    pub exdate: Vec<crate::DateTime>,
    pub rstatus: Vec<String>,
    pub related: Vec<String>,
    pub resources: Vec<String>,
    pub rdate: Vec<String>,
    pub x_prop: BTreeMap<String, String>,
    pub iana_prop: BTreeMap<String, String>,
}

impl VEvent {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl TryFrom<BTreeMap<String, String>> for VEvent {
    type Error = crate::Error;

    fn try_from(properties: BTreeMap<String, String>) -> crate::Result<Self> {
        let mut vevent = Self::new();

        for (key, value) in properties {
            match key.as_str() {
                "DTSTAMP" => vevent.dtstamp = crate::parser::parse_date(value)?,
                "UID" => vevent.uid = value,
                "DTSTART" => vevent.dtstart = crate::parser::parse_date(value)?,
                "DTEND" => vevent.dtend = crate::parser::parse_date(value)?,
                "DURATION" => {
                    vevent.dtend =
                        vevent.dtstart + crate::parser::parse_duration(value.as_str().into())?
                }
                "CLASS" => vevent.class = Some(value.into()),
                "CREATED" => vevent.created = Some(crate::parser::parse_date(value)?),
                "DESCRIPTION" => vevent.description = Some(value),
                "GEO" => vevent.geo = Some(value.try_into()?),
                "LAST-MODIFIED" => vevent.last_modified = Some(crate::parser::parse_date(value)?),
                "LOCATION" => vevent.location = Some(value),
                "ORGANIZER" => vevent.organizer = Some(crate::parser::parse_organizer(&value)?),
                "PRIORITY" => vevent.priority = Some(crate::parser::parse_priority(&value)?),
                "SEQ" => vevent.seq = Some(crate::parser::parse_sequence(&value)?),
                "STATUS" => vevent.status = Some(value.try_into()?),
                "SUMMARY" => vevent.summary = Some(value),
                "STRANSP" => vevent.transp = Some(value.try_into()?),
                "URL" => vevent.url = Some(value),
                "RECURID" => vevent.recurid = Some(crate::parser::parse_recurid(&value)?),
                "RRULE" => vevent.rrule = Some(value.try_into()?),
                "ATTACH" => vevent.attach.push(crate::parser::parse_attach(&value)),
                "ATTENDEE" => vevent.attendee.push(crate::parser::parse_attendee(&value)),
                "CATEGORIES" => vevent
                    .categories
                    .append(&mut crate::parser::parse_categories(&value)),
                "COMMENT" => vevent.comment.push(crate::parser::parse_comment(&value)),
                "CONTACT" => vevent.contact.push(crate::parser::parse_contact(&value)),
                "EXDATE" => vevent
                    .exdate
                    .append(&mut crate::parser::parse_exdate(&value)?),
                "RSTATUS" => vevent.rstatus.push(crate::parser::parse_rstatus(&value)?),
                "RELATED-TO" => vevent.related.push(crate::parser::parse_related(&value)),
                "RESOURCES" => vevent
                    .resources
                    .append(&mut crate::parser::parse_resources(&value)),
                "RDATE" => vevent
                    .rdate
                    .append(&mut crate::parser::parse_rdate(&value)?),
                _ => {
                    if key.starts_with("X-") {
                        vevent.x_prop.insert(key, value);
                    } else {
                        vevent.iana_prop.insert(key, value);
                    }
                }
            };
        }

        Ok(vevent)
    }
}

impl TryFrom<String> for VEvent {
    type Error = crate::Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        crate::parser::parse_vevent(raw.as_str().into())
            .map_err(crate::Error::from)
            .map(|(_, x)| x)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_vevent() {
        crate::test::test_files::<crate::VEvent>("events");
    }
}
