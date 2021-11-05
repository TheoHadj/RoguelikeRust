
// TO DO :
//     1-Voir le book "https://gameprogrammingpatterns.com/book" ! Implémenter des threads ? Choisir un pattern de programmation pour faire du clean
//     2-Implémenter une game loop plus propre


//     3-créer des obstacles
//     4-implémenter la map (qui deviendra une tileMap) et faire un algo qui la créer intelligement
//     5-monstres qui tuent quand on les touchent
//     6-IA pour les monstres
//     7-jeu en ligne ? avec server ? 

//     Après ça j'aurai fais un tour de ce que je peux faire







use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::*;
use std::time::Duration;


// mod object;
// use object::Player;


fn main()  {
  
    let mut player = Player {x : 5, y : 5, width : 32, height : 32};


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("test Roguelike", 800, 800)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().software().build().unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, 400, 300)
        .unwrap();
    

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut j=0;
    let mut i=0;


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {
                    keycode:Some(Keycode::Escape), ..
                } => {
                    break 'running
                },
                Event::KeyDown {
                    keycode:Some(Keycode::Z), .. 
                } => {
                    player.mouvement(0,-1);
                },
                Event::KeyDown {
                    keycode:Some(Keycode::Q), .. 
                } => {
                    player.mouvement(-1,0);
                },
                Event::KeyDown {
                    keycode:Some(Keycode::S), .. 
                } => {
                    player.mouvement(0,1);
                },
                Event::KeyDown {
                    keycode:Some(Keycode::D), .. 
                } => {
                    player.mouvement(1,0);
                },
                _ => {}

            }
        }
        i = (i+1) % 80;        

        // player.draw(canvas, creator);
        // player.x += 1; 
        // player.y += 1; 
        println!("{0} {1}", player.x, player.y);

        let rect = Some(Rect::new(player.x, player.y, player.width, player.height));
        canvas
            .with_texture_canvas(&mut texture, |texture_canvas|{
                texture_canvas.clear();
                texture_canvas.set_draw_color(Color::RGB(80 - i,175, 230));
                texture_canvas.fill_rect(Rect::new(0, 0, 400, 300)).unwrap();

        }).unwrap();
        canvas.set_draw_color(Color::RGB(50, 50, 70));
        canvas.clear();
        canvas.copy(&mut texture, None, rect).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    
    }   
    // player.SayPosition();
}




struct Game {
    map : TileMap
}

type TileMap = Vec<Vec<Tile>>;

pub struct Tile {

}
// TO DO (TILE)
// struct Map {
//     height: i32,
//     width: i32,

// }

// impl Map {
    
// }

// #[derive(Clone, Copy)]
pub struct Player{
    pub x:i32,
    pub y:i32,
    width: u32,
    height: u32,
    
}

impl Player {

    pub fn SayPosition(self)  {
        println!("X:{0}, Y:{1}", self.x, self.y)
    }


    pub fn mouvement(&mut self, x:i32, y:i32){

        self.x += x;
        self.y += y;
        println!("{0} !! {1}", self.x, self.y)
    }



  // pub fn init_texture(self, creator : TextureCreator<WindowContext>) -> Texture<'static>{
    //     let texture = creator
    //     .create_texture_target(PixelFormatEnum::RGBA8888, 400, 300)
    //     .unwrap();
    //     texture 

    // }

    // pub fn draw(self, canvas :WindowCanvas, creator : TextureCreator<WindowContext> ){
        
    //     let mut texture = creator
    //     .create_texture_target(PixelFormatEnum::RGBA8888, 250, 250)
    //     .unwrap();

    //     canvas
    //     .with_texture_canvas(&mut texture, |texture_canvas|{
    //         texture_canvas.clear();
    //         texture_canvas.set_draw_color(Color::RGB(70,70,70));
    //         texture_canvas.fill_rect(Rect::new(0, 0, 150, 250)).unwrap();

    // }).unwrap();
    // canvas.clear();
    // canvas.copy(&mut texture, None, Some(Rect::new(0, 0, 64, 64))).unwrap();
    // }


}


// struct Mouvement{
//     object : Player
// }


//Object attribut given as attributes to other class (player ...)
struct Position{ //+Taille ? 
    x:i32,
    y:i32,
    height: i32,
    width: i32,

}
struct Object{
    position:Position,
    block : bool,
}
struct Monster{
    position:Position,

}