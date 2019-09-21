use std::time::Instant;

pub struct FrameCounter {
    count: f32,
    beginning: Instant,
}

impl FrameCounter {
    pub fn new() -> FrameCounter {
        FrameCounter {
            count: 0f32,
            beginning: Instant::now(),
        }
    }

    pub fn as_string(&self) -> String {
        let fps = self.count as f32 * 1000.0 / self.beginning.elapsed().as_millis() as f32;
        format!("frame: {}, fps: {:.2}", self.count, fps)
    }

    pub fn step(&mut self) {
        self.count += 1f32;
    }
}
