extern crate hyper;
extern crate multipart;
extern crate regex;
extern crate flate2;
extern crate byteorder;

use std::io::Read;
use std::io::Write;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;
use hyper::Client;
use multipart::client::lazy::Multipart;
use regex::Regex;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use byteorder::{LittleEndian, WriteBytesExt};

fn read_gpx(filename: &str) -> String {
    let mut gpx_data = String::new();
    let gpx_file = File::open(filename)
        .expect("Could not open GPX file for reading");
    BufReader::new(gpx_file)
        .read_to_string(&mut gpx_data)
        .expect("Could not read GPX file data");
    gpx_data
}

fn convert_gpx(filename: &str) -> String {
    let client = Client::new();
    let gpx_data = read_gpx(filename);
    let mut html = String::new();
    let mut csv = String::new();

    Multipart::new()
        .add_text("convert_format", "text")
        .add_stream("uploaded_file_1",
                    gpx_data.as_bytes(),
                    Option::Some(filename),
                    None)
        .add_text("convert_delimiter", "comma")
        .add_text("convert_add_speed", "1")
        .add_text("convert_add_slope", "1")
        .add_text("add_elevation", "SRTM1")
        .add_text("units", "metric")
        .add_text("submitted", "Convert")
        .client_request(&client, "http://www.gpsvisualizer.com/convert?output")
        .expect("Could not convert GPX data on GPSVisualizer website")
        .read_to_string(&mut html)
        .expect("Could not read GPSVisualizer response");;

    let re = Regex::new(r"(/download/convert/[0-9]+\-[0-9]+\-data\.csv)")
        .expect("Could not bulid regex");
    if let Some(cap) = re.captures_iter(&html).next() {
        let url = "http://www.gpsvisualizer.com/".to_string() + &cap[0];
        client
            .get(&url)
            .send()
            .expect("Could not download CSV from GPSVisualizer")
            .read_to_string(&mut csv)
            .expect("Could not read downloaded CSV from GPSVisualizer");
    }

    csv
}

fn parse_csv(csv: &str) -> Vec<(f64,f64)> {
    let lines: Vec<_> = csv.split("\n").collect();
    let mut data = Vec::<(f64,f64)>::with_capacity(lines.len());

    for line in lines {
        let mut values = line.split(",");
        match values.nth(0) {
            Some(t) => {
                if t != "T" {
                    continue;
                }
            }
            None => continue,
        }

        let corrected_speed: f64;
        match values.nth(4) {
            Some("") => continue,
            Some(speed) => {
                corrected_speed = speed
                    .parse::<f64>()
                    .expect("Invalid value for speed in CSV");
            }
            None => continue,
        }

        let mut corrected_slope: f64 = 0.0;
        match values.next() {
            Some(slope) => {
                if let Ok(value) = slope.parse::<f64>() {
                    corrected_slope = value;
                }
            }
            None => continue,
        }

        data.push((corrected_speed, corrected_slope));
    }
    data
}

fn write_mat(filename: &str, data: &[(f64,f64)]) -> std::io::Result<()> {
    let mat = File::create(filename)
        .expect("Could not create MAT file");
    let mut writer = BufWriter::new(mat);

    // Header with description, version and endian info
    let header_text = "MATLAB 5.0 MAT-file, Platform: PCWIN64".to_string();
    writer.write_all(header_text.as_bytes())?;
    writer.write_all(" ".repeat(116 - header_text.len()).as_bytes())?;
    writer.write_all(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])?;
    writer.write_all(&[0x00, 0x01, 0x49, 0x4d])?;

    // miMatrix
    let mut gz = ZlibEncoder::new(Vec::new(), Compression::Default);
    gz.write(&[0x0e, 0x00, 0x00, 0x00])?;
    gz.write_u32::<LittleEndian>(data.len() as u32 * 16 + 48)?;

    // miUINT32, array flags
    gz.write(&[0x06, 0x00, 0x00, 0x00])?;
    gz.write(&[0x08, 0x00, 0x00, 0x00])?;
    gz.write(&[0x06, 0x00, 0x00, 0x00])?;
    gz.write(&[0x00, 0x00, 0x00, 0x00])?;

    // miINT32
    gz.write(&[0x05, 0x00, 0x00, 0x00])?;
    gz.write(&[0x08, 0x00, 0x00, 0x00])?;
    gz.write_u32::<LittleEndian>(data.len() as u32)?;
    gz.write(&[0x02, 0x00, 0x00, 0x00])?;

    // miINT8, var name
    gz.write(&[0x01, 0x00, 0x03, 0x00, 0x74, 0x78, 0x74, 0x00])?;

    // miDOUBLE, actual data
    gz.write(&[0x09, 0x00, 0x00, 0x00])?;
    gz.write_u32::<LittleEndian>(data.len() as u32 * 16)?;
    for &(speed, _) in data {
        gz.write_f64::<LittleEndian>(speed)?;
    }
    for &(_, slope) in data {
        gz.write_f64::<LittleEndian>(slope)?;
    }

    // Compressed data output
    let compressed_data = gz.finish()?;
    writer.write_all(&[0x0f, 0x00, 0x00, 0x00])?;
    writer.write_u32::<LittleEndian>(compressed_data.len() as u32)?;
    writer.write_all(&compressed_data)?;
    Ok(())
}

fn main() {
    let csv = convert_gpx("/home/cluosh/work/gps/gps/test.gpx");
    let data = parse_csv(&csv);
    write_mat("/home/cluosh/work/gps/gps/test.mat", &data)
        .expect("Failed to write MAT file");
}
