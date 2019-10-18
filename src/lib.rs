//! 🦇 BATS! 🦇

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tarcieri/bats/batcave/img/cutebat.png",
    html_root_url = "https://docs.rs/bats/0.0.0"
)]
#![warn(missing_docs, rust_2018_idioms)]

use crossterm_cursor::cursor;
use crossterm_terminal::terminal;
use gumdrop::Options;
use rand::{thread_rng, Rng};
use std::{thread::sleep, time::Duration};

/// Bat!
pub const BAT: char = '🦇';

/// Crab!
pub const CRAB: char = '🦀';

/// Halloween things
pub const HALLOWEEN_THINGS: &[char] = &['🦇', '🎃', '👻', '💀', '🕸', '🧙'];

/// Bats! A spooky bat printer
#[derive(Debug, Default, Options)]
pub struct Bats {
    /// Help
    #[options(help = "print help message")]
    help: bool,

    /// Character to print
    #[options(short = "c", long = "char", help = "character to print")]
    char: Option<char>,

    /// Draw crabs
    #[options(long = "crab", help = "draw crabs")]
    crab: bool,

    /// Speed
    #[options(
        short = "s",
        long = "speed",
        help = "speed factor (default 15, max 255)"
    )]
    pub speed: Option<u8>,

    /// Enable Halloween mode
    #[options(long = "halloween", help = "enable halloween mode")]
    pub halloween: bool,
}

impl Bats {
    /// Run the program
    pub fn run(&self) {
        println!("🦇 BATS! 🦇");
        sleep(Duration::from_millis(250));

        terminal()
            .clear(crossterm_terminal::ClearType::All)
            .unwrap();

        let cursor = cursor();
        cursor.hide().unwrap();
        cursor.goto(0, 0).unwrap();

        let thing = self.thing_to_draw();
        let is_halloween = self.is_it_halloween();

        loop {
            if is_halloween {
                self.draw_halloween();
            } else {
                self.draw(thing);
            }
        }
    }

    /// Draw a random halloweeny-thing
    pub fn draw_halloween(&self) {
        self.draw(HALLOWEEN_THINGS[thread_rng().gen_range(0, HALLOWEEN_THINGS.len() - 1)]);
    }

    /// Draw an arbitrary string
    pub fn draw(&self, thing: char) {
        let mut rng = thread_rng();
        let terminal = terminal();
        let (term_width, term_height) = terminal.size().unwrap();

        let cursor = cursor();
        let y_position = cursor.pos().unwrap().1;

        let start_pos = term_width - 2;
        let end_pos = rng.gen_range(0, start_pos);

        let mut delay = u64::from(10 + (start_pos - end_pos) * 2);
        let delay_scale = rng.gen_range(self.speed_factor(), self.speed_factor() * 2);

        for pos in (end_pos..start_pos).rev() {
            cursor.goto(pos, y_position).unwrap();
            terminal.write(thing).unwrap();

            sleep(Duration::from_millis(delay / delay_scale));
            delay -= 1;

            if pos != end_pos {
                cursor.goto(pos, y_position).unwrap();
                terminal.write(" ").unwrap();
            }
        }

        if y_position < term_height - 1 {
            cursor.goto(0, y_position + 1).unwrap();
        } else {
            terminal.clear(crossterm_terminal::ClearType::All).unwrap();
            cursor.goto(0, 0).unwrap();
        }

        sleep(Duration::from_millis(256 / self.speed_factor()));
    }

    /// Character to draw
    fn thing_to_draw(&self) -> char {
        if self.crab {
            if self.char.is_none() {
                CRAB
            } else {
                panic!("both --char and --crab options passed");
            }
        } else {
            self.char.unwrap_or(BAT)
        }
    }

    /// Get the current speed factor
    fn speed_factor(&self) -> u64 {
        u64::from(self.speed.unwrap_or(15))
    }

    /// Is it halloween?
    fn is_it_halloween(&self) -> bool {
        if self.halloween {
            return true;
        }

        use chrono::Datelike;
        let today = chrono::Local::today();
        today.month() == 10 && today.day() == 31
    }
}
