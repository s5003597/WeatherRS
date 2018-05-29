extern crate reqwest;
extern crate serde_json;

use reqwest::get;
use serde_json::Value;
use std::io::{stdin, stdout, Write};

const GAPI: &'static str = "Google API Here";
const DAPI: &'static str = "Darksky API Here";

fn main() {
    let user_choice: String = location_input();
    let (lat, long): (f64, f64) = google_req(user_choice);
    darksky_req(lat, long);
}

fn location_input() -> String {
    let mut input = String::new();
    print!("Location: ");
    let _= stdout().flush();
    stdin().read_line(&mut input).ok();
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    input
}

fn google_req(location: String) -> (f64, f64) {
    let g_url = format!("https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
                location,
                GAPI);
    
    let resp: Value = get(&g_url).unwrap().json().unwrap();

    (
        resp["results"][0]["geometry"]["location"]["lat"].as_f64().unwrap(),
        resp["results"][0]["geometry"]["location"]["lng"].as_f64().unwrap()
    )
}

fn darksky_req(lat: f64, long: f64) {
    let exclude: String = "minutely,daily,hourly,alerts,flags".to_string();

    let d_url = format!("https://api.darksky.net/forecast/{}/{},{}?exclude={}&units=uk2",
                DAPI,
                lat,
                long,
                exclude);

    let resp: Value = get(&d_url).unwrap().json().unwrap();
    println!("      Current Weather - {}", resp["currently"]["summary"].as_str().unwrap());
    println!("      Temperature - {}°C", resp["currently"]["temperature"].as_f64().unwrap());
    println!("      Humidity - {}", resp["currently"]["humidity"].as_f64().unwrap());
    println!("      Wind Speed - {} Mph", resp["currently"]["windSpeed"].as_f64().unwrap());
}
