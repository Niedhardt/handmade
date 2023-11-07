mod audio_driver;
mod display_driver;

use std::f32::consts::PI;
use std::time::{Duration, Instant, SystemTime};
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::{Event};
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::sys::on_exit;
use sdl2::TimerSubsystem;
use sdl2::video::Window;
use crate::audio_driver::{gen_wave, SineWave, SquareWave};
use crate::display_driver::DisplayDriver;

fn main() {

    let sdl_context = sdl2::init().expect("unable to init sdl2");
    let mut display = DisplayDriver::new(&sdl_context, 700, 500);
    let mut context = GameContext::new();

    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: None,
        channels: Some(1),
        samples: None,
    };

    /*
    let mut device_sine = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        SineWave {
            phase_inc: 200.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.005,
            test: 0,
        }
    }).unwrap();*/
    let device_sine = audio_subsystem.open_queue::<i16, _>(None, &desired_spec).unwrap();

    let target_bytes = 48_000 * 4;
    let wave = gen_wave(target_bytes);
    device_sine.queue_audio(&wave).unwrap();
    /*
    let mut device_sine = audio_subsystem.open_playback(None, &desired_spec, |spec| {
       SquareWave {
           phase_inc: 200.0 / spec.freq as f32,
           phase: 0.0,
           volume: 0.001,
       }
    }).unwrap();*/

   //// device_sine.resume();

    let mut x_offset: u32 = 0;
    let mut y_offset: u32 = 0;

    let mut sound_duration = 60;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {

        let now = SystemTime::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {
                    ..
                } => break 'running,
                Event::KeyDown {scancode: Some(scancode),..} => {
                    match scancode {
                        Scancode::W => {
                            sound_duration = 60;
                            //device_sine.
                            //device_sine.lock().volume = 0.003;
                            device_sine.resume();
                            y_offset += 12
                        },
                        Scancode::D => {
                            if x_offset > 0 {
                                x_offset -= 1;
                            }
                        },
                        Scancode::S => {
                            if y_offset > 0 {
                                y_offset -= 1;
                            }
                        },
                        Scancode::A => x_offset += 12,
                        Scancode::V => (), //device_sine.lock().volume += 0.0001,
                        Scancode::Escape => break 'running,
                        _ => {}
                    }
                },
                _ => {}
            }
        }

       // sound_duration -= 1;
        if sound_duration == 0 {
            device_sine.pause();
        }
        display.draw_something(x_offset, y_offset);
        display.resize();
        x_offset += 1;
        y_offset += 1;


        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32/240));
        match now.elapsed() {
            Ok(elapsed) => {
                //println!("ms: {}", elapsed.as_millis());
            }
            Err(e) => {
                println!("Error: {e:?}");
            }
        }
    }
}

struct GameContext {

}

impl GameContext{
    pub fn new() -> Self {
        Self {}
    }

    pub fn tick(&mut self) {
    }
}






