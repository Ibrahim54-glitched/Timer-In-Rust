use regex::Regex;
use std::io::Write;
use std::{io, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut input_timer = String::new();
    println!("Entering layout: XXh:XXm:XXs");
    println!("Timer: ");

    io::stdin()
        .read_line(&mut input_timer)
        .expect("Failed to read input");

    let input_timer = input_timer.trim();
    // A methon/function from regrex library which can split the string into many parts.
    // Let's understand down what's happening.
    // "?P<>" this syntax saparate the previous string the input <> character come, in this case
    // its h. \d+ it matches on or more digits present in the string from 0-9.
    let re = Regex::new(r"(?:(?P<h>\d+)h)?[:]?((?P<m>\d+)m)?[:]?((?P<s>\d+)s)?").unwrap();

    // Seperate the hours, minutes and seconds and assign the value to a variable respectively.
    let mut hours: u32 = 0;
    let mut minutes: u32 = 0;
    let mut seconds: u32 = 0;

    if let Some(caps) = re.captures(&input_timer) {
        hours = caps.name("h").map_or(0, |m| m.as_str().parse().unwrap());
        minutes = caps.name("m").map_or(0, |m| m.as_str().parse().unwrap());
        seconds = caps.name("s").map_or(0, |m| m.as_str().parse().unwrap());
    }

    println!(
        "The Timer is starting for {}h:{}m:{}s...",
        hours, minutes, seconds
    );

    let total_seconds = hours * 3600 + minutes * 60 + seconds;

    for remaining in (1..=total_seconds).rev() {
        let h = remaining / 3600;
        let m = (remaining % 3600) / 60;
        let s = remaining % 60;
        println!("Time remaining: {:02}:{:02}:{:02}", h, m, s);
        sleep(Duration::from_secs(1)).await;
    }

    println!("Timer expired!");
}
