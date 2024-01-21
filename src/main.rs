use bytes::Bytes;
use clap::Parser;
use encoding_rs::WINDOWS_1251;
use quick_xml::de::from_str;
use reqwest::blocking::{get, Response};
use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use std::fmt;

const CBRF_DAILY_XML_ENDPOINT: &str = "https://www.cbr.ru/scripts/XML_daily.asp";

/// Converts currency rates from XML to JSON format
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct ValCurs {
    #[serde(rename(deserialize = "@Date"))]
    date: String,
    #[serde(rename(deserialize = "@name"))]
    name: String,
    #[serde(rename(deserialize = "Valute"))]
    valutes: Vec<Valute>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Eq, Serialize)]
struct Valute {
    #[serde(rename(deserialize = "@ID"))]
    id: String,
    #[serde(rename(deserialize = "NumCode"))]
    num_code: String,
    #[serde(rename(deserialize = "CharCode"))]
    char_code: String,
    #[serde(rename(deserialize = "Nominal"))]
    nominal: u32,
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "Value"))]
    value: String,
    #[serde(rename(deserialize = "VunitRate"))]
    vunit_rate: String,
}

impl fmt::Display for ValCurs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:=^79}", " REPORT ")?;
        writeln!(f, "Date: {}\nName: {}", self.date, self.name)?;
        writeln!(f, "{:-^79}", "")?;
        writeln!(
            f,
            "{:<8} {:<8} {:<6} {:<8} {:<10} {:<12} {:<10}",
            "id", "num", "char", "nominal", "value", "vunit_rate", "name"
        )?;
        writeln!(f, "{:-^79}", "")?;
        for v in &self.valutes {
            write!(f, "{}", v)?;
        }
        Ok(())
    }
}

impl fmt::Display for Valute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{:<8} {:<8} {:<6} {:<8} {:<10} {:<12} {:<10}",
            self.id,
            self.num_code,
            self.char_code,
            self.nominal,
            self.value,
            self.vunit_rate,
            self.name
        )
    }
}

impl Ord for Valute {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.char_code.cmp(&other.char_code)
    }
}

impl PartialOrd for Valute {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Valute {
    fn eq(&self, other: &Self) -> bool {
        self.char_code == other.char_code
    }
}

fn get_data(url: &str) -> Result<Bytes, reqwest::Error> {
    let response: Response = get(url).expect("Failed to fetch XML from the internet");
    response.bytes()
}

fn decode_windows1251(bytes: &Bytes) -> String {
    let (decoded, _, _) = WINDOWS_1251.decode(bytes);
    decoded.into_owned()
}

fn main() {
    let cli = Cli::parse();

    let content = get_data(CBRF_DAILY_XML_ENDPOINT).expect("Failed to download from URL");
    let decoded_content: String = decode_windows1251(&content);
    let mut valcurs: ValCurs = from_str(&decoded_content).expect("Failed to parse XML");
    valcurs.valutes.sort();

    let res_json: String = serde_json::to_string(&valcurs).expect("Can't serialize to JSON");

    if cli.debug {
        println!("{}", valcurs);
    } else {
        println!("{}", res_json);
    }
}
