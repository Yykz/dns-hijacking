use clap::Parser;
use regex::Regex;
use std::net::Ipv4Addr;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Options {
    /// Regex that matches which domains are redirected
    pub regex: Regex,
    /// IP where the targeted domains are redirected
    pub ip: Ipv4Addr,
    /// Increase verbosity, and can be used multiple times
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

pub fn parse() -> Options {
    Options::parse()
}
