use std::env;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct SharedState {
    index: usize,
    turn: usize,
    cycles_completed: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: <string> <char_count> <thread_count>");
        return;
    }

    let input = args[1].clone();
    let chars: Vec<char> = input.chars().collect();
    let char_count: usize = args[2].parse().unwrap();
    let thread_count: usize = args[3].parse().unwrap();

    let max_cycles = 3; // change this for more/less cycles

    println!(
        "Using {} threads to print {} alternatively in set of {} chars for {} cycles",
        thread_count, input, char_count, max_cycles
    );

    let state = Arc::new((
        Mutex::new(SharedState {
            index: 0,
            turn: 0,
            cycles_completed: 0,
        }),
        Condvar::new(),
    ));
    let chars = Arc::new(chars);

    let mut handles = vec![];

    for thread_id in 0..thread_count {
        let state = Arc::clone(&state);
        let chars = Arc::clone(&chars);
        let thread_name = format!("Thread{}", thread_id + 1);

        let handle = thread::spawn(move || {
            loop {
                let (lock, cvar) = &*state;
                let mut shared = lock.lock().unwrap();

                while shared.turn != thread_id && shared.cycles_completed < max_cycles {
                    shared = cvar.wait(shared).unwrap();
                }

                if shared.cycles_completed >= max_cycles {
                    break;
                }

                // Print chunk
                let mut output = String::new();
                for _ in 0..char_count {
                    output.push(chars[shared.index % chars.len()]);
                    shared.index += 1;
                }

                println!("{}: {}", thread_name, output);

                // Update turn
                shared.turn = (shared.turn + 1) % thread_count;

                // If one full cycle done
                if shared.turn == 0 {
                    shared.cycles_completed += 1;
                }

                cvar.notify_all();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
