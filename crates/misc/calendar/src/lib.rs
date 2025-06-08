//! Calendrical algorithms.

/// Returns whether `y` is a leap year.
/// - Divisible by 400: yes.
/// - Divisible by 100, but not 400: no.
/// - Divisibly by 4, but not 100: yes.
///
/// # Example
/// ```
/// # use calendar::is_leap_year;
/// assert!(!is_leap_year(2002));
/// assert!(is_leap_year(2004));
/// assert!(!is_leap_year(2100));
/// assert!(is_leap_year(2400));
/// ```
pub fn is_leap_year(y: u32) -> bool {
    y % (if y % 25 == 0 { 16 } else { 4 }) == 0
}

/// Returns the last day of month `m` in year `y`.
///
/// ⚠️ Panics if not `1 <= m <= 12`.
///
/// # Example
/// ```
/// # use calendar::last_day_of_month;
/// assert_eq!(last_day_of_month(2010, 7), 31);
/// assert_eq!(last_day_of_month(2024, 2), 29);
/// ```
pub fn last_day_of_month(y: u32, m: u8) -> u8 {
    assert!((1..=12).contains(&m), "{}", m);
    if m == 2 {
        if is_leap_year(y) {
            29
        } else {
            28
        }
    } else {
        [0, 31, 0, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][m as usize]
    }
}

/// Returns whether year `y`, month `m`, and day `d`
/// constitutes a valid date.
///
/// # Example
/// ```
/// # use calendar::is_valid_date;
/// assert!(is_valid_date(2025, 6, 6));
/// assert!(!is_valid_date(2025, 2, 29));
/// assert!(!is_valid_date(2025, 0, 1));
/// ```
pub fn is_valid_date(y: u32, m: u8, d: u8) -> bool {
    (1..=12).contains(&m) && 0 < d && d <= last_day_of_month(y, m)
}
