mod domain;

use crate::domain::stats::Stat;
use crate::domain::user::User;
use chrono::{ NaiveDate, Duration };

fn main() {
    let mut user = User::new(String::from("Janko Hrasko"));
    let stat1 = Stat::new(180.0, 100.0, 100.0, 100.0);
    let stat2 = Stat::new(180.0, 120.0, 130.0, 100.0);
    let stat3 = Stat::new(200.0, 60.0, 200.0, 200.0);
    let stat4 = Stat::new(200.0, 150.0, 100.0, 200.0);

    let dt = NaiveDate::from_ymd(2014, 7, 8);
    let dt2 = dt - Duration::days(31);

    user.add_stat(dt, stat1);
    user.add_stat(dt - Duration::days(1), stat2);
    user.add_stat(dt2, stat3);
    user.add_stat(dt - Duration::days(365), stat4);
    println!("{:?}", user);
    println!("{:?}", user.get_current_stats().unwrap());
    println!("{:?}", user.get_stat_by_date(&dt2).unwrap());
    println!("{:?}", user.get_difference(&Duration::days(1)));
    println!("{:?}", user.get_difference(&Duration::days(31)));
    println!("{:?}", user.get_difference(&Duration::days(365)));
    println!("{:?}", user.get_difference(&Duration::days(5)));
    println!("{:?}", user.get_difference(&Duration::days(0)));
}
