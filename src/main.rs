extern crate hyper;
extern crate multipart;
extern crate regex;

use std::io::Read;

fn convert_gpx(filename: &str) -> String {
    use hyper::Client;
    use multipart::client::lazy::Multipart;
    use std::fs::File;
    use std::io::BufReader;

    let gpx_file = File::open(filename)
        .expect("Could not open GPX file for reading");
    let mut buf_reader = BufReader::new(gpx_file);
    let mut gpx = String::new();
    buf_reader.read_to_string(&mut gpx);

    let hyper_client = Client::new();
    let mut response = Multipart::new()
        .add_text("convert_format", "text")
        .add_stream("uploaded_file_1",
                    gpx.as_bytes(),
                    Option::Some(filename),
                    None)
        .add_text("convert_add_speed", "1")
        .add_text("convert_add_slope", "1")
        .add_text("add_elevation", "SRTM1")
        .add_text("units", "metric")
        .add_text("submitted", "Convert")
        .add_text("convert_delimiter", "")
        // .add_text("data", "name,desc,latitude,longitude")
        // .add_text("force_type", "")
        // .add_text("remote_data", "")
        // .add_text("convert_delimiter", "")
        // .add_text("vmg_point", "")
        // .add_text("add_elevation", "SRTM1")
        // .add_text("show_trk", "1")
        // .add_text("reverse", "0")
        // .add_text("connect_segments", "0")
        // .add_text("trk_merge", "0")
        // .add_text("trk_distance_threshold", "")
        // .add_text("trk_simplify", "")
        // .add_text("trk_stats", "0")
        // .add_text("trk_elevation_threshold", "")
        // .add_text("tickmark_interval", "")
        // .add_text("trk_as_wpt", "0")
        // .add_text("trk_as_wpt_name", "")
        // .add_text("trk_as_wpt_desc", "")
        // .add_text("convert_gpx_styles", "")
        // .add_text("trk_segment_time", "")
        // .add_text("add_timestamps", "")
        // .add_text("show_wpt", "3")
        // .add_text("synthesize_name", "")
        // .add_text("synthesize_desc", "")
        // .add_text("reference_point", "")
        // .add_text("reference_point_name", "")
        // .add_text("wpt_interpolate", "1")
        // .add_text("wpt_interpolate_offset", "")
        // .add_text("convert_repeat_headers", "1")
        // .add_text("time_offset", "")
        // .add_text("utm_output", "0")
        // .add_text("moving_average", "1")
        // .add_text("frequency_count", "none")
        // .add_text("special", "")
        // .add_text("cumulative_distance", "0")
        // .add_text("tickmark_zero", "1")
        // .add_text("wifi_mode", "3")
        // .add_text("forerunner_laps", "1")
        // .add_text("gps_altitude", "1")
        // .add_text("trk_preserve_attr", "1")
        // .add_text("wpt_preserve_attr", "1")
        // .add_text("convert_routes", "t_aw")
        // .add_text("padding", "10")
        // .add_text("wpt_name_filter", "")
        // .add_text("wpt_desc_filter", "")
        // .add_text("convert_add_climb", "")
        // .add_text("convert_add_slope_degrees", "")
        // .add_text("trk_reorder", "")
        // .add_text("trk_reorder_merge", "")
        // .add_text("wpt_polygons", "")
        .client_request(&hyper_client, "http://www.gpsvisualizer.com/convert?output")
        .expect("Could not convert GPX data on GPSVisualizer website");

    let mut text = "".to_string();
    response.read_to_string(&mut text).expect("Could not read GPSVisualizer response");

    let re = regex::Regex::new(r"(/download/convert/[0-9]+\-[0-9]+\-data\.txt)").unwrap();
    if let Some(cap) = re.captures_iter(&text).next() {
        let url = "http://www.gpsvisualizer.com/".to_string() + &cap[0];
        let mut response = hyper_client.get(&url).send().unwrap();

        let mut text = "".to_string();
        response.read_to_string(&mut text).expect("Could not read GPSVisualizer response");
        println!("{}", &text);
    }

    "DONE".to_string()
}

fn main() {
    println!("{}", convert_gpx("/home/cluosh/work/gps/gps/test.gpx"));
}
