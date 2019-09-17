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
        let fps = self.count / self.beginning.elapsed().as_secs_f32();
        format!("frame: {}, fps: {:.2}", self.count, fps)
    }

    pub fn step(&mut self) {
        self.count += 1f32;
    }
}
