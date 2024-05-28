use chrono::{DateTime, Utc};
use macaddr::MacAddr6;
use sha256::digest;

/// Calculate a hash from a username, date and mac address.
/// This function returns a string with the username and the hash separated by a dash.
/// ```rust
/// use exchan::user_to_hash;
/// use chrono::{DateTime, TimeZone, Utc};
/// use macaddr::MacAddr6;
///
/// let date: DateTime<Utc> = Utc.with_ymd_and_hms(2024, 03, 01, 9, 0, 0).unwrap(); // 2024-03-01T09:00:00Z
/// let macaddr: MacAddr6 = MacAddr6::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
/// let hostname: &str = "testhost";
/// let username: &str = "testuser";
/// let val = user_to_hash(username, date, macaddr, hostname);
/// assert_eq!(
///     val,
///     "testuser@testhost:e4268f3d770dd62d26eb6f57edd37e7bf427fa510bfc72cb4992005bfa7ad904"
/// );
/// ```
pub fn user_to_hash(
    username: &str,
    date: DateTime<Utc>,
    macaddr: MacAddr6,
    hostname: &str,
) -> String {
    let data: String = format!("{}{}{}{}", username, date, macaddr, hostname);
    let hash: String = digest(data);
    format!("{}@{}:{}", username, hostname, hash)
}

/// test module
#[cfg(test)]
mod tests {
    use super::user_to_hash;
    use chrono::{DateTime, TimeZone, Utc};
    use macaddr::MacAddr6;

    #[test]
    fn test_user_to_hash() {
        let date: DateTime<Utc> = Utc.with_ymd_and_hms(2024, 03, 01, 9, 0, 0).unwrap(); // 2024-03-01T09:00:00Z
        let macaddr: MacAddr6 = MacAddr6::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
        let hostname: &str = "testhost";
        let username: &str = "testuser";
        let val = user_to_hash(username, date, macaddr, hostname);
        assert_eq!(
            val,
            "testuser@testhost:e4268f3d770dd62d26eb6f57edd37e7bf427fa510bfc72cb4992005bfa7ad904"
        );
    }
}
