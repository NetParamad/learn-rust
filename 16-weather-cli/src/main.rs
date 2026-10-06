// 16 - weather CLI: cargo run -- "Bangkok"
// โน้ต: fetch 2 ต่อ (geocode -> forecast), error_for_status แทนเช็ค res.ok
use serde::Deserialize;

#[derive(Deserialize)]
struct GeoResponse {
    results: Option<Vec<GeoPlace>>,
}

#[derive(Deserialize)]
struct GeoPlace {
    name: String,
    country: Option<String>,
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize)]
struct Forecast {
    current: Current,
}

#[derive(Deserialize)]
struct Current {
    temperature_2m: f64,
    weather_code: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let city = env_arg();
    let client = reqwest::blocking::Client::new();

    // ต่อที่ 1: ชื่อเมือง -> พิกัด (.query() encode URL ให้เอง)
    let geo: GeoResponse = client
        .get("https://geocoding-api.open-meteo.com/v1/search")
        .query(&[("name", city.as_str()), ("count", "1")])
        .send()?
        .error_for_status()?
        .json()?;

    let place = geo
        .results
        .and_then(|mut r| r.pop())
        .ok_or(format!("city '{city}' not found"))?;

    // ต่อที่ 2: พิกัด -> อากาศปัจจุบัน
    let fc: Forecast = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&[
            ("latitude", place.latitude.to_string()),
            ("longitude", place.longitude.to_string()),
            ("current", "temperature_2m,weather_code".to_string()),
        ])
        .send()?
        .error_for_status()?
        .json()?;

    println!(
        "{} ({}): {:.1}C, weather code {}",
        place.name,
        place.country.unwrap_or_default(),
        fc.current.temperature_2m,
        fc.current.weather_code
    );
    Ok(())
}

fn env_arg() -> String {
    std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("Bangkok"))
}
