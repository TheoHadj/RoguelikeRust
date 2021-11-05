use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::*;

struct Position{x:i32,y:i32} //Commencez par du simple sans code reuse puis refactorer trait ? Inheritance ? 

    
pub struct Player{
    pub PositionX:i32,
    pub PositionY:i32,
    // height: i32,
    // width: i32,

}

impl Player {

    pub fn SayPosition(self)  {
        println!("X:{0}, Y:{1}", self.PositionX, self.PositionY)
    }

}



struct Object{
    PositionX:i32,
    PositionY:i32,
    height: i32,
    width: i32,

}
struct Monster{
    PositionX:i32,
    PositionY:i32,
    height: i32,
    width: i32,

}