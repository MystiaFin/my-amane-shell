use amane::{Argument, Bus, Service, Value};

const PORTAL: &str = "org.freedesktop.portal.Desktop";
const PORTAL_PATH: &str = "/org/freedesktop/portal/desktop";
const SETTINGS: &str = "org.freedesktop.portal.Settings";

const NAMESPACE: &str = "org.freedesktop.appearance";
const KEY: &str = "color-scheme";

// the light or dark preference the desktop announces, none when it has no preference
#[derive(Default)]
pub struct System {
    pub light: Option<bool>,
}

impl Service for System {
    fn new() -> Self {
        let bus = Bus::session();

        let value = bus.call(
            PORTAL,
            PORTAL_PATH,
            SETTINGS,
            "ReadOne",
            &[Argument::Text(String::from(NAMESPACE)), Argument::Text(String::from(KEY))],
        );

        Self {
            light: preference(&value),
        }
    }

    // the portal announces every change, so no polling
    fn listen() {
        let bus = Bus::session();

        for signal in bus.signals(SETTINGS, "SettingChanged") {
            let [namespace, key, value] = signal.arguments() else {
                continue;
            };

            if namespace.text() != NAMESPACE || key.text() != KEY {
                continue;
            }

            Self::write().light = preference(value);
        }
    }
}

// 0 is no preference, 1 prefers dark and 2 prefers light
fn preference(value: &Value) -> Option<bool> {
    match value.number() as u32 {
        1 => Some(false),
        2 => Some(true),
        _ => None,
    }
}
