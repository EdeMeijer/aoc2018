use std::collections::HashMap;

use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::Timelike;
use chrono::TimeZone;
use regex::Regex;

use utils::data::load_data;
use utils::data::non_empty_lines;

type Date = DateTime<Local>;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

#[derive(Debug, Eq, PartialEq)]
struct DateEvent {
    date: Date,
    event: Event,
}

#[derive(Debug, Eq, PartialEq)]
enum Event {
    BeginsShift { guard_id: u32 },
    FallsAsleep,
    WakesUp,
}

fn solve_part1(events: Vec<DateEvent>) -> u32 {
    let spg = get_sleeps_per_guard(events);

    let guard_with_most_sleeps = spg.iter()
        .max_by_key(|(_, s)| s.len())
        .unwrap()
        .0;

    let sleeps = &spg[guard_with_most_sleeps];

    let mut count_per_minute = HashMap::new();
    for date in sleeps {
        *count_per_minute.entry(date.minute()).or_insert(0) += 1;
    }

    let most_likely_minute = count_per_minute.iter()
        .max_by_key(|(_, c)| *c)
        .unwrap()
        .0;

    return guard_with_most_sleeps * most_likely_minute;
}


/// Collect all minutes of sleep per guard ID
fn get_sleeps_per_guard(events: Vec<DateEvent>) -> HashMap<u32, Vec<Date>> {
    let events = sort_events(events);

    let mut result = HashMap::new();

    let mut cur_guard = None;
    let mut sleeping_since = None;

    for evt in events {
        match evt.event {
            Event::BeginsShift { guard_id } => cur_guard = Some(guard_id),
            Event::FallsAsleep => sleeping_since = Some(evt.date),
            Event::WakesUp => result
                .entry(cur_guard.unwrap())
                .or_insert(vec![])
                .extend(minutes_between(sleeping_since.unwrap(), evt.date))
        }
    }

    result
}

/// Sort events by date
fn sort_events(mut events: Vec<DateEvent>) -> Vec<DateEvent> {
    events.sort_by_key(|e| e.date);
    events
}

/// Produce a list of every minute between two dates
fn minutes_between(from: Date, to: Date) -> Vec<Date> {
    let mut result = vec![];
    let mut cur = from;

    let one_minute = Duration::minutes(1);

    while cur < to {
        result.push(cur);
        cur = cur + one_minute;
    }

    result
}

fn get_puzzle_input() -> Vec<DateEvent> {
    parse_puzzle_input(load_data("day4"))
}

fn parse_puzzle_input(input: String) -> Vec<DateEvent> {
    non_empty_lines(input)
        .into_iter()
        .map(parse_event)
        .collect()
}

/// Parse the puzzle input representation of an event into a DateEvent object
fn parse_event(event: String) -> DateEvent {
    let re = Regex::new(
        r"^\[1518-0*(\d+)-0*(\d+) 0*(\d+):0*(\d+)] (falls|wakes|Guard) (?:#(\d+))?"
    ).unwrap();

    let cap = re.captures(&event).expect(&event);

    DateEvent {
        date: local_date(
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
            cap[4].parse().unwrap(),
        ),
        event: match &cap[5] {
            "Guard" => Event::BeginsShift { guard_id: cap[6].parse().unwrap() },
            "falls" => Event::FallsAsleep,
            "wakes" => Event::WakesUp,
            _ => panic!("Usupported event")
        },
    }
}

fn local_date(month: u32, day: u32, hour: u32, minute: u32) -> Date {
    Local.ymd(1518, month, day).and_hms(hour, minute, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_event() {
        assert_eq!(
            parse_event(String::from("[1518-11-01 00:00] Guard #10 begins shift")),
            DateEvent {
                date: local_date(11, 1, 0, 0),
                event: Event::BeginsShift { guard_id: 10 },
            }
        );

        assert_eq!(
            parse_event(String::from("[1518-11-04 00:36] falls asleep")),
            DateEvent {
                date: local_date(11, 4, 0, 36),
                event: Event::FallsAsleep,
            }
        );

        assert_eq!(
            parse_event(String::from("[1518-11-05 23:55] wakes up")),
            DateEvent {
                date: local_date(11, 5, 23, 55),
                event: Event::WakesUp,
            }
        );
    }

    #[test]
    fn test_part1() {
        let raw_input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        let input = parse_puzzle_input(raw_input.to_owned());

        assert_eq!(
            solve_part1(input),
            240
        )
    }
}
