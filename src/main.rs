use reqwest::blocking::{get, Response};
use quick_xml::de::from_str;
use serde::Deserialize;
use encoding_rs::WINDOWS_1251;
use std::cmp::Ord;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ValCurs {
    #[serde(rename = "@Date")]
    date: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "Valute")]
    valutes: Vec<Valute>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Eq)]
struct Valute {
    #[serde(rename = "@ID")]
    id: String,
    #[serde(rename = "NumCode")]
    num_code: String,
    #[serde(rename = "CharCode")]
    char_code: String,
    #[serde(rename = "Nominal")]
    nominal: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Value")]
    value: String,
    #[serde(rename = "VunitRate")]
    vunit_rate: String,
}

impl ValCurs {
    fn report(&self) {
        println!("{:=^79}", " REPORT ");
        println!("Date: {}", self.date);
        println!("Name: {}", self.name);
        println!("{:=^79}", "");
        for i in 0..self.valutes.len() {
            self.valutes[i].report();
        }    
    }
}

impl Valute {
    fn report(&self) {
        println!("- {}", self.name);
        
        println!("Id: {}", self.id);
        println!("NumCode: {}", self.num_code);
        println!("CharCode: {}", self.char_code);
        println!("Nominal: {}", self.nominal);
        println!("Value: {}", self.value);
        println!("UnitRate: {}", self.vunit_rate);
        

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

fn main() {
    let url: &str = "https://www.cbr.ru/scripts/XML_daily.asp"; // Replace with the actual URL
    let response: Response = get(url).expect("Failed to fetch XML from the internet");
    let bytes= response.bytes().unwrap();
    let xml_string: String = decode_windows1251(&bytes);
    println!("{:?}", xml_string);

    let mut valcurs: ValCurs = from_str(&xml_string).expect("Failed to parse XML");
    valcurs.valutes.sort();
    valcurs.report()
}

fn decode_windows1251(bytes: &[u8]) -> String {
    let (decoded, _, _) = WINDOWS_1251.decode(bytes);
    decoded.into_owned()
}

