use hickory_proto::rr::{rdata, record_data::RData, record_type::RecordType};
use std::{
    error::Error,
    fmt::Display,
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use clap::Parser;
use regex::Regex;

#[derive(Debug, Clone)]
struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid input")
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

fn rdata_from_str(
    rtype: &str,
    replacement: Option<&str>,
) -> Result<RData, Box<dyn Error + Send + Sync + 'static>> {
    match RecordType::from_str(rtype)? {
        RecordType::A => {
            let ip = Ipv4Addr::from_str(replacement.unwrap_or("127.0.0.1"))?;
            Ok(RData::A(rdata::A(ip)))
        }
        RecordType::AAAA => {
            let ip = Ipv6Addr::from_str(replacement.unwrap_or("::1"))?;
            Ok(RData::AAAA(rdata::AAAA(ip)))
        }
        _ => Err(ParseError)?,
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub regex: Regex,
    pub rdata: RData,
}

impl Entry {
    fn parse(entry: &str) -> Result<Entry, Box<dyn Error + Send + Sync + 'static>> {
        let mut entry = entry.split(';');
        let regex = Regex::new(entry.next().ok_or(ParseError)?)?;

        let rdata = rdata_from_str(
            entry.next().ok_or(ParseError)?,
            entry.next(),
        )?;

        Ok(Self { regex, rdata })
    }
}

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Options {
    #[clap(value_parser = Entry::parse, num_args = 1.., value_delimiter = ',')]
    /// List of entries that you want to redirect. They must be comma-separated, and each entry consists of a domain, rtype, and IP (which is local if you leave blank), separated by ';'.
    /// 
    /// Example: "google.com;A","example.com;AAAA;::1" redirects domains that match 'google.com' with IPv4 and domains that match 'example.com' with IPv6 to local.
    pub entries: Vec<Entry>,
    /// Increase verbosity, and can be used multiple times
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// Time To Live of fake answer
    #[arg(short, long, default_value_t = 0)]
    pub ttl: u32,
}

pub fn parse() -> Options {
    Options::parse()
}
