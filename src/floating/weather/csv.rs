use std::collections::HashMap;

/*
 * open-meteo's csv comes in blocks split by empty lines, each a header row
 * like "time,temperature_2m (°C)" over a row of values; this keeps the first
 * row of every block by name, without the unit in brackets
 */
pub fn values(text: &str) -> HashMap<String, String> {
    let mut values = HashMap::new();

    for block in text.split("\n\n") {
        let mut lines = block.lines();

        let (Some(header), Some(row)) = (lines.next(), lines.next()) else {
            continue;
        };

        for (name, value) in header.split(',').zip(row.split(',')) {
            let name = name.split(" (").next().unwrap_or(name).trim();

            values.insert(String::from(name), String::from(value.trim()));
        }
    }

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_every_block() {
        let text = "latitude,longitude\n-6.2,106.8\n\n\
                    time,temperature_2m (°C),uv_index ()\n2026-09-30T15:45,31.4,3.40\n\n\
                    time,temperature_2m_max (°C)\n2026-09-30,35.0\n2026-10-01,34.0\n";

        let parsed = values(text);

        assert_eq!(parsed["temperature_2m"], "31.4");
        assert_eq!(parsed["uv_index"], "3.40");

        // the first day, not the second
        assert_eq!(parsed["temperature_2m_max"], "35.0");

        assert!(values("").is_empty());
    }
}
