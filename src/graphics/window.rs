extern crate sdl2;
use self::sdl2::rect::Point;
use self::sdl2::pixels::Color;
use self::sdl2::Sdl;
use self::sdl2::video::FullscreenType;
use graphics::pixel::Pixel;
use graphics::texture::*;
use self::sdl2::rect::Rect;

pub struct Window {
    title: String,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Window {
    pub fn new(context: &Sdl, res: (u32,u32), title: String) -> Window {
        let vid = (context).video().unwrap();
        let canvas = vid.window(title.as_str(),
            res.0,
            res.1
        ).position_centered().build().unwrap()
            .into_canvas().build().unwrap();
        Window {
            title,
            canvas
        }

    }

    pub fn resize(&mut self, size: (u32,u32)) {
        let mut window = self.canvas.window_mut();
        window.set_size(size.0, size.1).unwrap();
    }

    pub fn fullscreen(&mut self, fullscreen_on: bool) {
        let mut window = self.canvas.window_mut();
        window.set_fullscreen(if fullscreen_on {
            FullscreenType::True
        } else {
            FullscreenType::Desktop
        }).unwrap();
    }

    pub fn is_open(&self) -> bool {
        true
    }

    pub fn draw_pixels(&mut self, pixels: &Vec<Vec<Pixel>>) {
        self.fill_blank();
        for i in pixels {
            for q in i {
                self.canvas.set_draw_color(Color::RGB(q.rbg.0,q.rbg.1,q.rbg.2));
                self.canvas.draw_point::<Point>(q.to_sdl_point()).expect("Sawblade Error: could not draw to window successfully");
            }
        }
        self.canvas.present();
    }

    pub fn draw_texture(&mut self, texture: FinalTexture) {
        //let texture_creator = self.canvas.texture_creator();
        match texture.get_texture() {
            SawbladeTexture::Rect(width, height) => {
                self.canvas.set_draw_color(Color::RGB(255,0,255));
                let rect = Rect::new(texture.get_coordinates().0 as i32, texture.get_coordinates().1 as i32, width, height);
                self.canvas.fill_rect(rect).expect("Could not draw rectangle to SDL2 canvas");
            },
            /*
            SawbladeTexture::Circle(radius) => {

            }
            SawbladeTexture::FromFile(file_name) => {

            }
            */
            _ => ()
        };
        self.canvas.set_draw_color(Color::RGB(255,255,255));
    }

    pub fn update(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0,0,0));
        self.canvas.present();
    }

    pub fn fill_blank(&mut self) {
        self.canvas.clear();
    }

    pub fn close(&mut self) {
        self.canvas.clear();
    }

    pub fn get_canvas(&mut self) -> &mut sdl2::render::Canvas<sdl2::video::Window> {
        &mut self.canvas
    }
}
