/**
 * See [3.8.1.11. Status](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.11)
 */
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Status {
    /** Indicates event is tentative */
    Tentative,
    /** Indicates event is definite */
    Confirmed,
    /** Indicates event/to-do/journal was cancelled/removed */
    Cancelled,
    /** Indicates to-do needs action */
    NeedsAction,
    /** Indicates to-do completed */
    Completed,
    /** Indicates to-do in process of */
    InProcess,
    /** Indicates journal is draft */
    Draft,
    /** Indicates journal is final */
    Final,
}

impl TryFrom<String> for Status {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for Status {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::str::FromStr for Status {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        let status = match s {
            "TENTATIVE" => Self::Tentative,
            "CONFIRMED" => Self::Confirmed,
            "CANCELLED" => Self::Cancelled,
            "NEEDS-ACTION" => Self::NeedsAction,
            "COMPLETED" => Self::Completed,
            "IN-PROCESS" => Self::InProcess,
            "DRAFT" => Self::Draft,
            "FINAL" => Self::Final,

            _ => return Err(crate::Error::Status(s.to_string())),
        };

        Ok(status)
    }
}
