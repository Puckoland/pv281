use crate::domain::stats::Stat;
use std::collections::BTreeMap;
use chrono::{ Duration, NaiveDate };

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub stats: BTreeMap<NaiveDate, Stat>,
}

impl User {
    /// Create a new User from String
    ///
    /// # Examples
    /// ```
    /// use crate::bodovana::domain::user::User;
    /// let user = User::new(String::from("Janko Hrasko"));
    /// assert_eq!(user.name, "Janko Hrasko");
    /// ```
    pub fn new(name: String) -> User {
        User { name: name, stats: BTreeMap::new() }
    }

    /// Add an existing Stat to this User
    /// 
    /// # Examples
    /// ```
    /// use crate::bodovana::domain::user::User;
    /// use crate::bodovana::domain::stats::Stat;
    /// use chrono::NaiveDate;
    /// let dt = NaiveDate::from_ymd(2021, 10, 22);
    /// let mut user = User::new(String::from("Janko Hrasko"));
    /// let stat = Stat::new(180.0, 100.0, 100.0, 100.0);
    /// user.add_stat(dt, stat);
    /// assert!(user.stats.get(&dt).unwrap().height == 180.0);
    /// ```
    pub fn add_stat(self: &mut User, date: NaiveDate, stat: Stat) {
        self.stats.insert(date, stat);
    }

    /// Returns the most recently added stat.
    pub fn get_current_stats(&self) -> Option<(&NaiveDate, &Stat)> {
        self.stats.iter().next_back()
    }

    /// Get statistics for given date.
    /// Returns None if there is no statistics for given date.
    pub fn get_stat_by_date(&self, date: &NaiveDate) -> Option<&Stat> {
        self.stats.get(date)
    }

    /// Returns difference between most recent statistics and statistics for given date in form of new statistics.
    /// If any of the statistics does not exist, returns None.
    pub fn get_difference(&self, duration: &Duration) -> Option<Stat> {
        match self.get_current_stats() {
            Some((curr_date, curr_stat)) => {
                let old_date = *curr_date - *duration;
                match self.get_stat_by_date(&old_date) {
                    Some(old_stat) => Some(*curr_stat - *old_stat),
                    None => None,
                }
            },
            None => None,
        }
    }
}
