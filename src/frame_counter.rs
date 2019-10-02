use std::time::Instant;

pub struct FrameCounter<'a> {
    name: &'a str,
    speed_unit: &'a str,
    count: f32,
    beginning: Instant,
}

impl FrameCounter<'_> {
    pub fn new<'a>(name: &'a str, speed_unit: &'a str) -> FrameCounter<'a> {
        FrameCounter {
            name: name,
            speed_unit: speed_unit,
            count: 0f32,
            beginning: Instant::now(),
        }
    }

    pub fn as_string(&self) -> String {
        let speed = self.count as f32 * 1000.0 / self.beginning.elapsed().as_millis() as f32;
        format!(
            "{}: {}, {:.2} {}",
            self.name, self.count, speed, self.speed_unit
        )
    }

    pub fn step(&mut self) {
        self.count += 1f32;
    }
}
