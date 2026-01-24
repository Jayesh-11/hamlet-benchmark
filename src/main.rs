mod constants;
use crate::constants::all_chars::ALL_CHARS;
use crate::constants::hamlet::HAMLET;
use clap::Parser;
use std::io::{self, Write};

use rand::Rng;
use std::{thread, time};
/// Simple program benchmark by printing hamlet
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Italics
    #[arg(short, long, default_value_t = true)]
    italics: bool,

    /// Delay in ms on each iteration
    #[arg(short, long, default_value_t = 0)]
    delay: u64,

    /// Enable random colors for each iteration
    #[arg(short, long, default_value_t = false)]
    random_color_mode: bool,

    /// Just print, no randomness
    #[arg(short, long, default_value_t = false)]
    just_print: bool,
}

fn main() {
    let args = Args::parse();
    const ALL_CHARS_INDEX_LENGTH: usize = ALL_CHARS.len() - 1;

    let start_time = time::Instant::now();
    let mut iterations = 0;

    for c in HAMLET.chars() {
        if args.just_print == true {
            if args.delay > 0 {
                thread::sleep(time::Duration::from_millis(args.delay));
            }
            iterations += 1;
            print!("{}", c);
            io::stdout().flush().unwrap();
            continue;
        }

        let mut random_idx: usize = rand::rng().random_range(0..ALL_CHARS_INDEX_LENGTH);
        let mut random_char: char = ALL_CHARS.chars().nth(random_idx).unwrap();
        while c != random_char {
            random_idx = rand::rng().random_range(0..ALL_CHARS_INDEX_LENGTH);
            random_char = ALL_CHARS.chars().nth(random_idx).unwrap();
            iterations += 1;

            if args.delay > 0 {
                thread::sleep(time::Duration::from_millis(args.delay));
            }

            if random_char == '\t' || random_char == '\n' {
                continue;
            }

            if args.random_color_mode == true {
                let random_bright_color_idx: usize = rand::rng().random_range(40..231);
                print!("{}\x1b[48;5;{}m\x08", random_char, random_bright_color_idx);
            } else {
                print!("{}\x08", random_char);
            }

            io::stdout().flush().unwrap();
        }
        // print!("\x1b[1D");
        // https://stackoverflow.com/questions/53162888/is-there-an-ansi-control-sequence-which-moves-the-cursor-to-the-end-of-line
        // we need to move to end of line of previous line and remove the last character/tab(tab
        // will be somewhat more complex) and then add new \n

        if random_char == '\n' {
            print!(" ");
            io::stdout().flush().unwrap();
        }

        print!("{}\x1b[3m", random_char);
        io::stdout().flush().unwrap();
    }

    if args.delay > 0 {
        return;
    }

    let final_time = start_time.elapsed().as_millis();
    let iterations_per_ms = iterations / final_time;
    let performance_print: String = format!(
        r#"

    ------------------------
    |
    |  time: {final_time} ms
    |  iterations: {iterations}
    |  iterations/ms: {iterations_per_ms}
    |
    ------------------------
    "#
    );
    println!("{}", performance_print);
}
