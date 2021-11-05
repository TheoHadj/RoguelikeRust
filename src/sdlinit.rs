use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::*;
use std::time::Duration;


pub fn init(){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("test Roguelike", 800, 800)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().software().build().unwrap();
    let creator = canvas.texture_creator();

}