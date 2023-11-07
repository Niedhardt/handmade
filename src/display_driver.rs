use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct DisplayDriver {
    pub canvas: Canvas<Window>,
    pub width: u32,
    pub height: u32,
}

impl DisplayDriver {
    pub fn new(sdl_context: &sdl2::Sdl, width: u32, height: u32) -> Self {
        let video_subsys = sdl_context
            .video()
            .expect("unable to init video_subsys");

        let window = video_subsys
            .window("something", width, height)
            .position_centered()
            .resizable()
            .build()
            .expect("unable to create window");

        let mut canvas = window.into_canvas().build().expect("unable to create canvas");

        canvas.clear();
        canvas.present();

        Self {
            canvas,
            width,
            height,
        }
    }

    pub fn resize(&mut self) {
        let test = self.canvas.window_mut();
        let size = test.size();
        self.width = size.0;
        self.height = size.1;
    }
    pub fn draw_something(&mut self, x_offset: u32, y_offset: u32) {

        let texture_creator = self.canvas.texture_creator();

        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24,self.width, self.height)
            .map_err(|e| e.to_string()).unwrap();

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..self.height {
                for x in 0..self.width {

                    let offset = ((y) as usize * pitch) + ((x) as usize * 3);
                    //R
                    buffer[offset as usize] = 0;
                    //G
                    buffer[(offset + 1) as usize] = (x + x_offset) as u8;
                    //B
                    buffer[(offset + 2) as usize] = (y + y_offset) as u8;
                }
            }
        }).unwrap();

        self.canvas.clear();
        self.canvas.copy(&texture, None, Some(Rect::new(0, 0, self.width, self.width))).unwrap();
        /* self.canvas.copy_ex(
             &texture,
             None,
             Some(Rect::new(450, 100, self.width, self.height)),
             30.0,
             None,
             false,
             false
         ).unwrap();*/


        self.canvas.present();

    }

}