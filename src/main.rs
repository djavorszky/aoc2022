#![allow(unused)]

mod day1;
mod day2;
mod day3;
mod day4;

pub mod prelude {
    pub use anyhow::{anyhow, bail, Context, Error, Result};
}

use crate::prelude::*;

fn main() -> Result<()> {
    //day1::run()
    //day2::run()
    //day3::run()
    day4::run()
}
