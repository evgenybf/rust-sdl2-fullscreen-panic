#![allow(dead_code)]

extern crate sdl2;
extern crate sdl2_image;

mod framerate;

use std::path::Path;
//use sdl2::*;
use sdl2::pixels;
use sdl2::surface;
//use sdl2_image::*;
use sdl2::keycode::KeyCode;

use framerate::FPSManager;

const SCREEN_X: i32 = 320;
const SCREEN_Y: i32 = 240;

const TARGET_FRAMERATE: u32 =  60;
const MAX_FRAME_TIME: u32 = 5 * 1000 / TARGET_FRAMERATE;

pub fn main() {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO | sdl2::INIT_AUDIO | sdl2::INIT_TIMER).unwrap();
    sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG);

    let window = sdl2::video::Window::new(&sdl_context, "Super Transball 2 v1.5 (Rust)", sdl2::video::WindowPos::PosCentered, 
            sdl2::video::WindowPos::PosCentered, SCREEN_X, SCREEN_Y, sdl2::video::OPENGL | sdl2::video::FULLSCREEN).unwrap();

    let mut renderer = sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED).unwrap();

    let surface: surface::Surface = sdl2_image::LoadSurface::from_file(&Path::new("ice-troll.bmp")).unwrap();
    surface.set_color_key(true, pixels::Color::RGB(238,238,238)).unwrap();
    let texture = renderer.create_texture_from_surface(&surface).unwrap();

    let mut fps_manager = FPSManager::new(TARGET_FRAMERATE);
    
    let mut color_index = 0;
    let mut angle = 0;
    
    let mut event_pump = sdl_context.event_pump();

    'main : loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit {..} => {
                    break 'main
                },
                Event::KeyDown { keycode: KeyCode::Escape, .. } => {
                    break 'main
                },
                _ => {}
            }
        }
        
        let mut drawer = renderer.drawer();
        
        drawer.set_draw_color(sdl2::pixels::Color::RGB(color_index, color_index, color_index));
        drawer.clear();
        drawer.copy_ex(&texture, None, None, angle as f64, None, (false, false));
        
        drawer.present();
        
        color_index = (color_index + 1) % 255;
        angle = (angle + 1) % 360;
        
        fps_manager.delay();
    }

    sdl2_image::quit();
}

