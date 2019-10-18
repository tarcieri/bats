//! 🦇 BATS! 🦇
//!
//! CLI wrapper

use bats::Bats;
use gumdrop::Options;

fn main() {
    Bats::parse_args_default_or_exit().run();
}
