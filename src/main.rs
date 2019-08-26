//!
//! morse_code/src/main.rs
//!
//! Michael G. Cummings
//! 2019-08-26
//!
//! Since Rust doesn't have build-in audio support text output is used.
//!
// MIT License
//
// Copyright (c) 2019 Michael Cummings
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::process;
use structopt::StructOpt;
use morse_code::{Config, Opt, run};

/// Core of the command-line binary.
///
/// By default expects input from stdin and outputs resulting morse code to stdout, but can also
/// read and/or write to files.
/// Use `morse_code --help` for more information about options.
fn main() {
    let opts = Opt::from_args();
    let mut config = Config::new(opts).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(err) = run(&mut config) {
        eprintln!("Application error: {}", err);
        process::exit(2);
    }
}
