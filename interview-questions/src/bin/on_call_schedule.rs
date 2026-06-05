/*
Given on-call rotation schedule for multiple people by: their name, start time and end time of the rotation:

Abby 1 10
Ben 5 7
Carla 6 12
David 15 17

Your goal is to return rotation table without overlapping periods representing who is on call during that time.
Return "Start time", "End time" and list of on-call people:

[1, 5] : ["Abby"]
[5, 6] : ["Abby", "Ben"]
[6, 7] : ["Abby", "Ben", "Carla"]
[7, 10] : ["Abby", "Carla"]
[10, 12] : ["Carla"]
[15, 17] : ["David"]
*/

use std::collections::{BTreeMap, HashSet};

fn find_on_call(timings: Vec<(&str, i32, i32)>) {
    let mut line: BTreeMap<i32, Vec<&str>> = BTreeMap::new();

    for (name, start, end) in timings {
        line.entry(start).or_default().push(name);
        line.entry(end).or_default().push(name);
    }

    let mut current_oncall: HashSet<&str> = HashSet::new();
    let mut previous_time: Option<i32> = None;

    for (&time, names) in line.iter() {
        if let Some(prev) = previous_time {
            if !current_oncall.is_empty() {
                let mut people: Vec<_> = current_oncall.iter().cloned().collect();
                people.sort();
                println!("[{prev}, {time}] : {:?}", people);
            }
        }
        for &name in names {
            if current_oncall.contains(name) {
                current_oncall.remove(name);
            } else {
                current_oncall.insert(name);
            }
        }
        previous_time = Some(time);
        
    }
}

fn main() {
    let timings = vec![
        ("Abby", 1, 10),
        ("Ben", 5, 7),
        ("Carla", 6, 12),
        ("David", 15, 17),
    ];

    find_on_call(timings);
}