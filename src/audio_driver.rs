use std::f32::consts::PI;
use sdl2::audio::AudioCallback;

pub struct SquareWave {
    pub phase_inc: f32,
    pub phase: f32,
    pub volume: f32,
}


pub struct SineWave {
    pub phase_inc: f32,
    pub phase: f32,
    pub volume: f32,
    pub test: u8,
}

impl AudioCallback for SineWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in 0..out.len() {
            out[x] = 0.5 * (2.0 * PI * x as f32 / out.len() as f32).sin();
            //println!("{}", out[x]);
            //out[x] = 0.5 * (440.0 * 2.0 * PI * x as f32).sin();
            /*
            let val = 0.5 * (440.0 * 2.0 * PI * self.phase).sin();

            let vas = 10.0 * (2.0 * PI * self.phase_inc * self.phase + 0.0).sin();
            let v = 2.0 * PI * (self.phase_inc + self.phase);
            *x = vas * self.volume;
            println!("x: {}", x);
            self.phase = vas;*/
            //println!("phase: {}", self.phase);
        }
        if self.test == 0{
            println!("{:?}", out);
            self.test += 1;
        }
    }

}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub fn gen_wave(bytes_to_write: i32) -> Vec<i16> {
   let tone_volume = 1_000i16;
    let period = 48_000 / 256;
    let sample_count = bytes_to_write;

    let mut result = Vec::new();

    for x in 0..sample_count {
        result.push(
            if (x / period) % 2 == 0{
                tone_volume
            } else {
                -tone_volume
            });
    }

    result
}