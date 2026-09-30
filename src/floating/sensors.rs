use std::fs;
use std::time::Duration;

use amane::Service;

// hwmon names that report the cpu package, and ones that report a gpu of its own
const CPU_SENSORS: [&str; 3] = ["coretemp", "k10temp", "zenpower"];
const GPU_SENSORS: [&str; 3] = ["amdgpu", "nouveau", "radeon"];

// temperatures in whole degrees celsius, none when the machine has no such sensor
#[derive(Default)]
pub struct Sensors {
    pub cpu: Option<u32>,

    // an integrated gpu has no sensor of its own, the cpu's covers it
    pub gpu: Option<u32>,
}

// polled, the kernel only exposes sensors as files
impl Service for Sensors {
    fn new() -> Self {
        let mut sensors = Self::default();

        sensors.update();

        sensors
    }

    fn interval() -> Duration {
        Duration::from_secs(3)
    }

    fn update(&mut self) -> bool {
        let before = (self.cpu, self.gpu);

        self.cpu = read(&CPU_SENSORS);
        self.gpu = read(&GPU_SENSORS);

        (self.cpu, self.gpu) != before
    }
}

// the first sensor with one of these names, in millidegrees in its temp1_input
fn read(names: &[&str]) -> Option<u32> {
    let folders = fs::read_dir("/sys/class/hwmon").ok()?;

    for folder in folders.flatten() {
        let path = folder.path();

        let name = fs::read_to_string(path.join("name")).unwrap_or_default();

        if !names.contains(&name.trim()) {
            continue;
        }

        let millidegrees: u32 = fs::read_to_string(path.join("temp1_input"))
            .ok()?
            .trim()
            .parse()
            .ok()?;

        return Some(millidegrees / 1000);
    }

    None
}
