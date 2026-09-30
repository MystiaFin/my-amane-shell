use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use amane::{Argument, Bus};

const PORTAL: &str = "org.freedesktop.portal.Desktop";
const PORTAL_PATH: &str = "/org/freedesktop/portal/desktop";
const FILE_CHOOSER: &str = "org.freedesktop.portal.FileChooser";
const REQUEST: &str = "org.freedesktop.portal.Request";

// the only formats amane decodes
const PNG_SIGNATURE: [u8; 4] = [0x89, b'P', b'N', b'G'];
const JPEG_SIGNATURE: [u8; 3] = [0xFF, 0xD8, 0xFF];

/*
 * opens the desktop's own file picker and waits for the answer; none when
 * it is cancelled, when no portal runs, or when the file isn't a png or jpeg
 */
pub fn choose_image(title: &str) -> Option<PathBuf> {
    let bus = Bus::session();

    // listening starts before asking, so a quick answer is never missed
    let answers = bus.signals(REQUEST, "Response");

    let options = BTreeMap::from([(String::from("modal"), Argument::Bool(true))]);

    let arguments = [
        Argument::Text(String::new()),
        Argument::Text(String::from(title)),
        Argument::Map(options),
    ];

    let request = bus.call(PORTAL, PORTAL_PATH, FILE_CHOOSER, "OpenFile", &arguments);

    let request = String::from(request.text());

    if request.is_empty() {
        eprintln!("amane: no desktop portal answered, the picture can't be chosen");

        return None;
    }

    for answer in answers {
        if answer.path() != request {
            continue;
        }

        // 0 is a choice, 1 a cancel and 2 anything else
        let [response, results] = answer.arguments() else {
            return None;
        };

        if response.number() != 0.0 {
            return None;
        }

        let uri = results.get("uris").list().first()?.text();

        let path = from_uri(uri)?;

        return is_image(&path).then_some(path);
    }

    None
}

// "file:///home/me/My%20Picture.png" to /home/me/My Picture.png
fn from_uri(uri: &str) -> Option<PathBuf> {
    let encoded = uri.strip_prefix("file://")?;

    let bytes = encoded.as_bytes();

    let mut decoded = Vec::with_capacity(bytes.len());

    let mut index = 0;

    while index < bytes.len() {
        let escaped = bytes[index] == b'%' && index + 2 < bytes.len();

        let value = if escaped {
            let hex = std::str::from_utf8(&bytes[index + 1..index + 3]).ok();

            hex.and_then(|hex| u8::from_str_radix(hex, 16).ok())
        } else {
            None
        };

        match value {
            Some(value) => {
                decoded.push(value);

                index += 3;
            }

            None => {
                decoded.push(bytes[index]);

                index += 1;
            }
        }
    }

    let text = String::from_utf8(decoded).ok()?;

    Some(PathBuf::from(text))
}

// judged by the file's first bytes, the same way amane decodes it
fn is_image(path: &PathBuf) -> bool {
    let mut start = [0u8; 4];

    let Ok(mut file) = File::open(path) else {
        return false;
    };

    if file.read_exact(&mut start).is_err() {
        return false;
    }

    start == PNG_SIGNATURE || start[..3] == JPEG_SIGNATURE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_file_uris() {
        let path = from_uri("file:///home/me/My%20Picture%E2%9C%93.png");

        assert_eq!(path, Some(PathBuf::from("/home/me/My Picture✓.png")));

        assert_eq!(from_uri("https://example.com/a.png"), None);
    }
}
