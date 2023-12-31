use reqwest::blocking::Client;
use serde::Deserialize;
use quick_xml::de::Deserializer;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::io::BufRead;

#[derive(Deserialize, Debug)]
struct Currency {
    #[serde(rename = "CharCode")]
    char_code: String,
    #[serde(rename = "Nominal")]
    nominal: i16,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Value")]
    value: String,
}

fn process_currency(currency: &Currency) {
    println!("{}: {:>8} - {} ({})", currency.char_code, currency.value, currency.name, currency.nominal);
}


fn get_content(url: String) -> String {
    // get data from url
    let client: Client = Client::new();
    let result: Result<reqwest::blocking::Response, reqwest::Error> = client.get(url).send();

    match result {
        Ok(result) => return result.text().expect("lazha"),
        Err(error) => panic!("Problem opening URL: {:?}", error),
    };
}

fn read_to_end_into_buffer<R: BufRead>(
    reader: &mut Reader<R>,
    start_tag: &BytesStart,
    junk_buf: &mut Vec<u8>,
) -> Result<Vec<u8>, quick_xml::Error> {
    let mut depth = 0;
    let mut output_buf: Vec<u8> = Vec::new();
    let mut w = Writer::new(&mut output_buf);
    let tag_name = start_tag.name();
    w.write_event(Event::Start(start_tag.clone()))?;
    loop {
        junk_buf.clear();
        let event = reader.read_event_into(junk_buf)?;
        w.write_event(&event)?;

        match event {
            Event::Start(e) if e.name() == tag_name => depth += 1,
            Event::End(e) if e.name() == tag_name => {
                if depth == 0 {
                    return Ok(output_buf);
                }
                depth -= 1;
            }
            Event::Eof => {
                panic!("oh no")
            }
            _ => {}
        }
    }
}




fn main() {
    let url: String = "https://www.cbr.ru/scripts/XML_daily.asp".to_string();
    let content: String = get_content(url);

    let mut reader = Reader::from_str(&content);
    let mut buf = Vec::new();

    let mut junk_buf: Vec<u8> = Vec::new();
    let mut count = 0;


    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"Valute" => {
                    let release_bytes =
                        read_to_end_into_buffer(&mut reader, &e, &mut junk_buf).unwrap();
                    let str = std::str::from_utf8(&release_bytes).unwrap();
                    let mut deserializer = Deserializer::from_str(str);
                    let currency = Currency::deserialize(&mut deserializer).unwrap();
                    process_currency(&currency);
                    count += 1;
                    if count % 1_000_000 == 0 {
                        println!("checked {} records", count);
                    }
                }
                _ => (),
            },
            // other unimportant Events
            _ => (),
        }
        // clear buffer to avoid leaking memory
        buf.clear();
    }

    println!("checked {} records", count);
}
