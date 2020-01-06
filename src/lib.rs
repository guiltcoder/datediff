//! Utility to get date difference (Similar to SQL DATEDIFF)
//!
//! Datediff provides utility to get difference of two dates
//! as Interval i.e days, months and years.
//!
//! # Quick Start
//!
//! ```
//! use chrono::NaiveDate;
//!
//! use datediff::get_diff;
//!
//! let start_date = NaiveDate::from_ymd(1947, 8, 15);
//! let end_date = NaiveDate::from_ymd(1950, 1, 26);
//!
//! println!("Duration is {}", get_diff(&start_date, &end_date));
//!
//! # use datediff::Interval;
//! # let duration = get_diff(&start_date, &end_date);
//! # assert_eq!(duration.days(), 11);
//! # assert_eq!(duration.months(), 5);
//! # assert_eq!(duration.years(), 2);
//! # assert_eq!(duration.positive(), true);
//! ```
//!

use chrono::{Datelike, NaiveDate};

use std::{fmt, mem};

/// Gives no of days in a given month for given year.
/// It takes care of leap day as well. Panics for invalid input.
///
/// # Example
///
/// ```
/// use datediff::total_days_in_month;
///
/// let year = 2020;
/// let month = 2;
///
/// println!("No of Days: {}", total_days_in_month(year, month));
/// # assert_eq!(total_days_in_month(year, month), 29);
/// ```

pub fn total_days_in_month(year: i32, month: u32) -> u32 {
    if month == 12 {
        NaiveDate::from_ymd(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd(year, month + 1, 1)
    }
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days() as u32
}

/// Holds the difference in days, months, years.
/// ```positive``` flag tells whether the difference of two dates was positive or negative
#[derive(Debug, PartialEq)]
pub struct Interval {
    days: u32,
    months: u32,
    years: u32,
    positive: bool,
}

impl Interval {
    pub fn days(&self) -> u32 {
        self.days
    }
    pub fn months(&self) -> u32 {
        self.months
    }
    pub fn years(&self) -> u32 {
        self.years
    }
    pub fn positive(&self) -> bool {
        self.positive
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} years {} months {} days {})",
            self.years,
            self.months,
            self.days,
            if self.positive { "Ahead" } else { "Behind" }
        )
    }
}

/// Takes two ```chrono::NaiveDate``` as arguments to get the duration in between
/// and gives the difference as Interval
///
/// # Example
///
/// ```
/// use chrono::NaiveDate;
///
/// use datediff::get_diff;
///
/// let start_date = NaiveDate::from_ymd(1857, 1, 5);
/// let end_date = NaiveDate::from_ymd(2020, 1, 1);
///
/// println!("Duration is {}", get_diff(&start_date, &end_date));
///
///
/// # use datediff::Interval;
///
/// # let duration = get_diff(&start_date, &end_date);
/// # assert_eq!(duration.days(), 27);
/// # assert_eq!(duration.months(),11);
/// # assert_eq!(duration.years(), 162);
/// # assert_eq!(duration.positive(), true);
///
/// ```
pub fn get_diff(start: &NaiveDate, end: &NaiveDate) -> Interval {
    let mut positive = true;
    let (mut start, mut end) = (start.clone(), end.clone());
    if end < start {
        positive = false;
        mem::swap(&mut start, &mut end);
    }

    //Typecasting for calculations
    let (start_day, mut end_day) = (start.day() as i32, end.day() as i32);
    let (start_month, mut end_month) = (start.month() as i32, end.month() as i32);
    let (start_year, mut end_year) = (start.year(), end.year());

    if end_day < start_day {
        //borrow days from previous month
        if end_month > 1 {
            end_day += total_days_in_month(end_year, (end_month - 1) as u32) as i32;
        }
        //borrow days from last month of previous year
        else {
            end_day += total_days_in_month(end_year - 1, 12) as i32;
        }
        end_month -= 1;
    }
    if end_month < start_month {
        //borrow months from previous year
        end_month += 12;
        end_year -= 1;
    }

    Interval {
        days: (end_day - start_day) as u32,
        months: (end_month - start_month) as u32,
        years: (end_year - start_year) as u32,
        positive,
    }
}

#[cfg(test)]
mod test {
    use crate::{get_diff, Interval, total_days_in_month};
    use chrono::{NaiveDate, Utc};

    #[test]
    fn validate() {
        assert_eq!(
            get_diff(
                &NaiveDate::from_ymd(2020, 1, 1),
                &NaiveDate::from_ymd(2020, 1, 1)
            ),
            Interval {
                years: 0,
                months: 0,
                days: 0,
                positive: true
            }
        );

        assert_eq!(
            get_diff(
                &NaiveDate::from_ymd(2020, 2, 5),
                &NaiveDate::from_ymd(2013, 1, 1)
            ),
            Interval {
                years: 7,
                months: 1,
                days: 4,
                positive: false
            }
        );

        assert_eq!(
            get_diff(
                &NaiveDate::from_ymd(2013, 2, 5),
                &NaiveDate::from_ymd(2020, 1, 1)
            ),
            Interval {
                years: 6,
                months: 10,
                days: 27,
                positive: true
            }
        );

        assert_eq!(
            get_diff(
                &NaiveDate::from_ymd(3040, 3, 1),
                &NaiveDate::from_ymd(1102, 1, 5)
            ),
            Interval {
                years: 1938,
                months: 1,
                days: 25,
                positive: false
            }
        );

        // Leap Year
        assert_eq!(total_days_in_month(1752, 2), 29);

        assert_eq!(total_days_in_month(2019, 2), 28);

        assert_eq!(total_days_in_month(3016, 6), 30);
    }
}
