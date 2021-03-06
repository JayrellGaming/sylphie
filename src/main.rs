#![warn(unused_extern_crates)]
#![recursion_limit="128"]
#![feature(nll, never_type, integer_atomics, optin_builtin_traits, fnbox, const_fn,
           const_atomic_bool_new, const_atomic_usize_new, conservative_impl_trait,
           try_trait)]
#![deny(unused_must_use)]

// TODO: Clean up general program structure
// TODO: Clean up program thread usage
// TODO: Pass around IDs less to touch Serenity's cache less.
// TODO: Add statistics tracking to better understand current bot load.
// TODO: Add logging for verifications to a log channel.
// TODO: Rewrite to be async.

extern crate backtrace;
extern crate byteorder;
extern crate chrono;
extern crate constant_time_eq;
extern crate fs2;
extern crate hmac;
extern crate hyper;
extern crate linefeed;
extern crate num_cpus;
extern crate parking_lot;
extern crate percent_encoding;
extern crate r2d2;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate rusqlite;
extern crate serde_json;
extern crate serenity;
extern crate sha2;
extern crate threadpool;
extern crate uuid;

#[allow(unused_extern_crates)] extern crate serde;

#[macro_use] extern crate enumset;
#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

#[macro_use] mod errors;

mod commands;
mod core;
mod database;
mod error_report;
mod logger;
mod roblox;
mod startup;
mod util;

fn main() {
    println!("Sylph-Verifier v{} by LymeeFairy", env!("CARGO_PKG_VERSION"));
    println!("Licenced under the Apache license, version 2.0");
    println!();

    println!("{}", std::mem::size_of::<errors::Error>());

    startup::start();
    std::process::exit(0); // Just in case.
}
