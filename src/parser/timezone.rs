/**
 * See [3.8.3. Time Zone Component Properties](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3)
 */

/**
 * See [3.8.3.1. Time Zone Identifier](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.1)
 */
pub(crate) fn tzid(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.3.2. Time Zone Name](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.2)
 */
pub(crate) fn tzname(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.3.3. Time Zone Offset From](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.3)
 */
pub(crate) fn tzoffsetfrom(input: &str) -> crate::Result<chrono::offset::FixedOffset> {
    input.parse().map_err(crate::Error::from)
}

/**
 * See [3.8.3.4. Time Zone Offset To](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.4)
 */
pub(crate) fn tzoffsetto(input: &str) -> crate::Result<chrono::offset::FixedOffset> {
    input.parse().map_err(crate::Error::from)
}

/**
 * See [3.8.3.5. Time Zone URL](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.5)
 */
pub(crate) fn tzurl(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}