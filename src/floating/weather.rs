mod csv;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Duration;

use amane::Service;

const FORECAST: &str = "https://api.open-meteo.com/v1/forecast";
const AIR_QUALITY: &str = "https://air-quality-api.open-meteo.com/v1/air-quality";

// the latest reading for the saved location, empty until the first answer
#[derive(Default, PartialEq)]
pub struct Weather {
    pub place: String,

    pub temperature: Option<f32>,
    pub high: Option<f32>,
    pub low: Option<f32>,

    // a wmo weather code, like 3 for cloudy
    pub code: Option<u32>,

    pub humidity: Option<f32>,
    pub uv_index: Option<f32>,

    // the us air quality index, 0 to 500
    pub air_quality: Option<f32>,
}

/*
 * polled, and asked outside the lock: the answer comes over the network,
 * and nothing is written when it didn't change
 */
impl Service for Weather {
    fn new() -> Self {
        Self::default()
    }

    fn interval() -> Duration {
        Duration::from_secs(15 * 60)
    }

    fn listen() {
        loop {
            let fresh = fetch();

            if *Self::read() != fresh {
                *Self::write() = fresh;
            }

            std::thread::sleep(Self::interval());
        }
    }
}

impl Weather {
    pub fn condition(&self) -> &'static str {
        match self.code {
            Some(0) => "Clear sky",
            Some(1) => "Mainly clear",
            Some(2) => "Partly cloudy",
            Some(3) => "Cloudy",
            Some(45 | 48) => "Fog",
            Some(51 | 53 | 55) => "Drizzle",
            Some(56 | 57) => "Freezing drizzle",
            Some(61) => "Light rain",
            Some(63) => "Rain",
            Some(65) => "Heavy rain",
            Some(66 | 67) => "Freezing rain",
            Some(71 | 73 | 75 | 77) => "Snow",
            Some(80 | 81 | 82) => "Rain showers",
            Some(85 | 86) => "Snow showers",
            Some(95 | 96 | 99) => "Thunderstorm",
            _ => "Weather",
        }
    }

    // material design weather glyphs from the nerd font
    pub fn icon(&self) -> &'static str {
        match self.code.unwrap_or(3) {
            0 => "\u{f0599}",
            1 | 2 => "\u{f0595}",
            3 => "\u{f0590}",
            45 | 48 => "\u{f0591}",
            51..=57 | 80..=82 => "\u{f0596}",
            61..=67 => "\u{f0597}",
            71..=77 => "\u{f0598}",
            85 | 86 => "\u{f0f36}",
            95.. => "\u{f067e}",
            _ => "\u{f0590}",
        }
    }
}

// the saved location, like "latitude=-6.2" on its own line
struct Location {
    latitude: String,
    longitude: String,
    name: String,
}

fn fetch() -> Weather {
    let Some(location) = location() else {
        return Weather {
            place: String::from("Set a location"),
            ..Weather::default()
        };
    };

    let coordinates = format!("latitude={}&longitude={}", location.latitude, location.longitude);

    let forecast = get(&format!(
        "{FORECAST}?{coordinates}&current=temperature_2m,relative_humidity_2m,weather_code,uv_index\
         &daily=temperature_2m_max,temperature_2m_min&forecast_days=1&timezone=auto&format=csv"
    ));

    let air = get(&format!("{AIR_QUALITY}?{coordinates}&current=us_aqi&format=csv"));

    let number = |values: &HashMap<String, String>, name: &str| values.get(name)?.parse().ok();

    Weather {
        place: location.name,
        temperature: number(&forecast, "temperature_2m"),
        high: number(&forecast, "temperature_2m_max"),
        low: number(&forecast, "temperature_2m_min"),
        code: forecast.get("weather_code").and_then(|code| code.parse().ok()),
        humidity: number(&forecast, "relative_humidity_2m"),
        uv_index: number(&forecast, "uv_index"),
        air_quality: number(&air, "us_aqi"),
    }
}

// empty when offline; the next poll tries again
fn get(url: &str) -> HashMap<String, String> {
    let text = amane::output(&format!("curl -sf --max-time 20 '{url}'"));

    csv::values(&text)
}

fn location() -> Option<Location> {
    let home = env::var("HOME").ok()?;

    let saved = fs::read_to_string(format!("{home}/.local/state/amane/weather")).ok()?;

    let mut values = HashMap::new();

    for line in saved.lines() {
        if let Some((name, value)) = line.split_once('=') {
            values.insert(name.trim(), value.trim());
        }
    }

    // the coordinates go into a shell command, so only numbers are let through
    let number = |name: &str| {
        let value = values.get(name)?;

        value.parse::<f64>().ok()?;

        Some(String::from(*value))
    };

    Some(Location {
        latitude: number("latitude")?,
        longitude: number("longitude")?,
        name: String::from(*values.get("name").unwrap_or(&"Weather")),
    })
}
