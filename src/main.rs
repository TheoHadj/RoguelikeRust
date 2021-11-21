// TO DO :
//     1-Voir le book "https://gameprogrammingpatterns.com/book" ! Implémenter des threads ? Choisir un pattern de programmation pour faire du clean
//     2-Implémenter une game loop plus propre (treads )


//     3-créer des obstaclesV
//     4-implémenter la room (qui deviendra une TileMap) et faire un algo qui la créer intelligementV
//  4.5 - Changement de carte V/2
//     5-monstres qui tuent quand on les touchent
//     6-IA pour les monstres
//     7-Des projectiles
//     8-jeu en ligne ? avec server ? 
// Implémenter le tout en thread 
//     Après ça j'aurai fais un tour de ce que je peux faire


//  LA FN MAIN AVEC LES GRAPHISMES EST DANS GAME 

//  AJOUTER A DESIGN LES COULEURS


// "!!!" = CHOSES IMPORTANTES
// #[derive(Clone, Copy)]

// !!! passer toutes les variables en u32
// !!! meme plus petit qu'une tile le player fonde ses calculs sur les tiles RÉGLÉ
// Thread avec event ne venant pas de sdl2



use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Texture, TextureCreator};
use sdl2::rect::*;
use sdl2::video::WindowContext;
use sdl2::pixels::{Color, PixelFormatEnum};
use std::array;
use std::time::Duration;


// mod object;
// use object::Player;


fn main()  {
  
    let mut game = Game::new(512,512);
    let mut player = Player::new(150, 100, 16, 16);
    // player.addObjectToInventory();
    // println!("{}",player.inventory.len());
    let mut playernumdeux = Player::new( 130, 130, 32, 32);

    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("test Roguelike",game.ROOM_WIDTH as u32 , game.ROOM_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas  = window.into_canvas()
                                        .present_vsync()
                                        .build()
                                        .unwrap();
    let creator = canvas.texture_creator();
    
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut listOfProjectile : Vec<HitBox> = Vec::new();

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
                    player.mouvement(&game,0,-1);
                },
                Event::KeyDown {
                    keycode:Some(Keycode::Q), .. 
                } => {
                    player.mouvement(&game,-1,0);
                },
                Event::KeyDown {
                    keycode:Some(Keycode::S), .. 
                } => {
                    player.mouvement(&game,0,1);
                },
                Event::KeyDown {
                    keycode:Some(Keycode::D), .. 
                } => {
                    player.mouvement(&game,1,0);
                },
                Event::KeyDown {
                    keycode:Some(Keycode::Up), .. 
                } => {
                    player.mouvement(&game,0,-1);
                    player.mouvement(&game,0,-1);
                },
                Event::KeyDown {
                    keycode:Some(Keycode::Down), .. 
                } => {
                    player.mouvement(&game,0,1);
                    player.mouvement(&game,0,1);
                },
                Event::KeyDown {
                    keycode:Some(Keycode::Right), .. 
                } => {
                    player.mouvement(&game,1,0);
                    player.mouvement(&game,1,0);
                },
                Event::KeyDown {
                    keycode:Some(Keycode::Left), .. 
                } => {
                    player.mouvement(&game,-1,0);
                    player.mouvement(&game,-1,0);
                },
                Event::KeyDown {
                    keycode:Some(Keycode::Space), .. 
                } => {
                    let x = player.attack(&mut game);
                    listOfProjectile.push(x);
                },
                // Event::KeyDown {
                //     keycode:Some(Keycode::Up), .. 
                // } => {
                //     playernumdeux.mouvement(&game,0,-1); 
                // },
                // Event::KeyDown {
                //     keycode:Some(Keycode::Down), .. 
                // } => {
                //     playernumdeux.mouvement(&game,0,1);
                // },
                // Event::KeyDown {
                //     keycode:Some(Keycode::Right), .. 
                // } => {
                //     playernumdeux.mouvement(&game,1,0);
                // },
                // Event::KeyDown {
                //     keycode:Some(Keycode::Left), .. 
                // } => {
                //     playernumdeux.mouvement(&game,-1,0);
                // },
                _ => {}

            }
        }

        let mut ListObjects = [&player, &playernumdeux];
        let listDesignTile = game.getDesignFromTile(&player);
        let mut listOfElementToRemove : Vec<i8> = Vec::new() ;
        println!("{}", listOfProjectile.len()) ;


        for tile in &listDesignTile{
            let mut texture = tile.set_texture(&creator);
            let rect = Some(Rect::new(tile.x,tile.y, tile.width, tile.height));
            canvas
                .with_texture_canvas(&mut texture, |texture_canvas|{
                    texture_canvas.clear();
                    texture_canvas.set_draw_color(Color::RGB(tile.colour[0],tile.colour[1], tile.colour[2]));
                    texture_canvas.fill_rect(Rect::new(0, 0, 16, 16)).unwrap(); // !!!REVOIR CA CARRE DE 400 POUR TEXTURE DE 16 
    
            }).unwrap();
            canvas.set_draw_color(Color::RGB(50, 50, 70));
            canvas.copy(&mut texture, None, rect).unwrap();
    
        }


        //Transformer listObjects en Design (c'est déjà des joueurs)

        for object in ListObjects{
            let mut texture = object.design.set_texture(&creator);
            let rect = Some(Rect::new(object.design.x,object.design.y, object.design.width, object.design.height));
            canvas
                .with_texture_canvas(&mut texture, |texture_canvas|{
                    texture_canvas.clear();
                    texture_canvas.set_draw_color(Color::RGB(80,175, 230));
                    texture_canvas.fill_rect(Rect::new(0, 0, 400, 300)).unwrap();
    
            }).unwrap();
            canvas.set_draw_color(Color::RGB(50, 50, 70));
            canvas.copy(&mut texture, None, rect).unwrap();
    

        }
        
        // METTRE PROJECTCALCUL AILLEURS
        'projectcalcul : for object in &mut listOfProjectile{
            let mut i = 0;
            let mut blocked = false;
            let mut door = false;
            println!("X : {0} § Y : {1}", object.design.x, object.design.y);
            let positionTile = game.calculPositionTile(object.design.x + object.vitessex as i32, object.design.y + object.vitessey as i32, object.design.height, object.design.width);
            for xi in positionTile.xmin..(positionTile.xmax +1){ // !!! ATTENTION AU || !!! C'EST ICI QUE CE CREE LES PORTES INVISIBLES 
                if (game.map[player.map_x as usize][player.map_y as usize].tableRoom[(positionTile.ymin) as usize][(xi) as usize].blocked == true)||(game.map[player.map_x as usize][player.map_y as usize].tableRoom[(positionTile.ymax) as usize][(xi) as usize].blocked == true){
                    blocked = true;
                    println!("1");
                }
                else if (game.map[player.map_x as usize][player.map_y as usize].tableRoom[(positionTile.ymin) as usize][(xi) as usize].blocked == true)||(game.map[player.map_x as usize][player.map_y as usize].tableRoom[(xi) as usize][(positionTile.ymax) as usize].door == true) {
                    door = true;
                    println!("2");

                }
            }

            for yi in positionTile.ymin..(positionTile.ymax +1){
                if (game.map[player.map_x as usize][player.map_y as usize].tableRoom[(yi) as usize][(positionTile.xmin) as usize].blocked == true)||(game.map[player.map_x as usize][player.map_y as usize].tableRoom[(yi) as usize][(positionTile.xmax) as usize].blocked == true){
                    blocked = true;
                    println!("3");

                }
                else if (game.map[player.map_x as usize][player.map_y as usize].tableRoom[(yi) as usize][(positionTile.xmin) as usize].blocked == true)||(game.map[player.map_x as usize][player.map_y as usize].tableRoom[(yi) as usize][(positionTile.xmax) as usize].door == true) {
                    door = true;
                    println!("4");
                }
            }
            if blocked == true || door == true {
                listOfElementToRemove.push(i);
                println!("5");

                // listOfProjectile.remove(i);
                // i+=1;
                // continue 'projectcalcul;
                

            }
            else {
                object.design.x += object.vitessex as i32; 
                object.design.y += object.vitessey as i32; 
                println!("6");
            }

            i+=1;
        }
        
        for index in listOfElementToRemove{
            listOfProjectile.remove(index as usize);
        }

        for object in &listOfProjectile{
            let mut texture = object.design.set_texture(&creator);
            let rect = Some(Rect::new(object.design.x,object.design.y, object.design.width, object.design.height));
            canvas
                .with_texture_canvas(&mut texture, |texture_canvas|{
                    texture_canvas.clear();
                    texture_canvas.set_draw_color(Color::RGB(80,175, 230));
                    texture_canvas.fill_rect(Rect::new(0, 0, 400, 300)).unwrap();
    
            }).unwrap();
            canvas.set_draw_color(Color::RGB(object.design.colour[0], object.design.colour[1], object.design.colour[2]));
            canvas.copy(&mut texture, None, rect).unwrap();
    

        }


        canvas.present();
        canvas.clear();


        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    
    
    }   
    // player.SayPosition();
}




pub struct Game {
    // position on room ? Ou plutôt dans player !
    map : MapTileMap,
    // room : RoomTileMap,
    TileHeight : i32,
    ROOM_HEIGHT : i32,
    ROOM_WIDTH : i32,
    nbTileHeight : i32,
    nbTileWidth : i32,
    MAP_HEIGHT : i32,
    MAP_WIDTH : i32,
    listOfProjectile : Vec<HitBox>

}

impl Game{
    pub fn new(ROOM_HEIGHT :i32,ROOM_WIDTH:i32) -> Game{
        let TileHeight = 16;
        let MAP_HEIGHT = 3;
        let MAP_WIDTH = 3;
        let nbTileHeight = ROOM_HEIGHT/TileHeight;
        let nbTileWidth = ROOM_WIDTH/TileHeight;
        // let ROOMANTIBUG:RoomTileMap = vec![vec![Tile::empty(); (nbTileHeight) as usize]; (nbTileWidth) as usize]; // A CHANGER POUR AVOIR PLUSIEURS ROOM 
        let mut map = vec![vec![RoomTileMap::new(); (MAP_HEIGHT) as usize]; (MAP_WIDTH) as usize];
        let mut listOfProjectile = Vec::new(); 
        // let mut roomANTICRASH = vec![vec![Tile::empty(); (nbTileHeight) as usize]; (nbTileWidth) as usize]; // A CHANGER POUR AVOIR PLUSIEURS ROOM 
        // let mut room = vec![vec![Tile::empty(); (nbTileHeight) as usize]; (nbTileWidth) as usize]; // A CHANGER POUR AVOIR PLUSIEURS ROOM 
        
        // for x in 0..(nbTileHeight) as usize{
        //     room[x][0]= Tile::wall();
        //     room[x][(nbTileWidth -1)  as usize]= Tile::wall();
        // }
        
        // for y in 0..nbTileWidth as usize{
        //     room[0][y]= Tile::wall();
        //     room[(nbTileHeight -1) as usize][y]= Tile::wall();
        // }
        
        // room[7][3] = Tile::wall(); // !!! ATTENTION AU ROOM WIDTH IMPAIR
        // room[((ROOM_HEIGHT/TileHeight)/2 -1)as usize][0] = Tile::door();
        // // room[((ROOM_HEIGHT/TileHeight)/2 +1)as usize ][0] = Tile::empty();
        // room[((ROOM_HEIGHT/TileHeight)/2 -1)as usize][(ROOM_HEIGHT/TileHeight -1)as usize] = Tile::door();
        // room[0][((ROOM_WIDTH/TileHeight)/2 -1)as usize] = Tile::door();
        // room[(ROOM_WIDTH/TileHeight -1)as usize][((ROOM_WIDTH/TileHeight)/2 -1)as usize] = Tile::door();

        // // println!("{0}", room[((ROOM_HEIGHT/TileHeight)/2 )as usize][0].blocked);


        let game = Game {map : map,  TileHeight : TileHeight, ROOM_HEIGHT : ROOM_HEIGHT, ROOM_WIDTH : ROOM_WIDTH, nbTileHeight : nbTileHeight ,nbTileWidth: nbTileWidth, MAP_WIDTH :MAP_WIDTH, MAP_HEIGHT :MAP_HEIGHT, listOfProjectile:listOfProjectile};
        let mut map = vec![vec![RoomTileMap::empty(&game); (MAP_HEIGHT) as usize]; (MAP_WIDTH) as usize];
        map[0][1] = RoomTileMap::centerWall(&game);  
        map[1][1] = RoomTileMap::centerWall(&game);  
        map[1][2] = RoomTileMap::centerWall(&game);  
        let mut listOfProjectile = Vec::new(); 
        let game = Game {map : map,  TileHeight : TileHeight, ROOM_HEIGHT : ROOM_HEIGHT, ROOM_WIDTH : ROOM_WIDTH, nbTileHeight : nbTileHeight ,nbTileWidth: nbTileWidth, MAP_HEIGHT : MAP_HEIGHT, MAP_WIDTH : MAP_WIDTH, listOfProjectile:listOfProjectile};

        game
    }

    pub fn calculPositionTile(&self, x : i32, y : i32, height : u32, width : u32) -> Position{
        // Chaque Tile a les pixels de [x * self.TileHeight à ((x+1) * self.TileHeight) -1] C'est l'inverse
        // Passer en paramètre player et  remplacer chaque valeurs par la valeur dans player.design  
        let xMinTile = x/(self.TileHeight);
        let xMaxTile = (x+ (height as i32-1))/(self.TileHeight);
        let yMinTile = y/(self.TileHeight);
        let yMaxTile = (y+ (width as i32-1))/(self.TileHeight) ;
        let position = Position::new(xMinTile, yMinTile, xMaxTile, yMaxTile);
        position
    }
    
    //Mettre avec room
    // !!!  AJOUTER UNE COULEUR AU DESIGN POUR LES DIFFERENTS TYPES DE TILES 
    pub fn getDesignFromTile(&self, player : &Player) -> Vec<Design>  {
        let mut listDesign = Vec::new();
        let abc = &self.map[player.map_x as usize][player.map_y as usize];
        for x in 0..(self.ROOM_HEIGHT/self.TileHeight) as usize {
            for y in 0..(self.ROOM_WIDTH/self.TileHeight) as usize{
                if abc.tableRoom[x][y].blocked == true{
                    let design =  Design::new((y as i32)*self.TileHeight,(x as i32)*self.TileHeight,self.TileHeight as u32,self.TileHeight as u32,65,20,65);
                    listDesign.push(design);
                }
                if abc.tableRoom[x][y].door == true{
                    let design =  Design::new((y as i32)*self.TileHeight,(x as i32)*self.TileHeight,self.TileHeight as u32,self.TileHeight as u32,160,160,160);
                    listDesign.push(design);
                }
            }
        }
    

        

        listDesign
    }

    pub fn appendListProject(&mut self, hitBox : HitBox){
        // let hitBox = HitBox::new(); 
        self.listOfProjectile.push(hitBox);

        
    }


}


// type RoomTileMap = Vec<Vec<Tile>>; // !!! Vec vec int de 0 à n puis enum qui les relie à des objets Tile => 0 -> Tile.empty   1 -> Tile.wall  VOIR VIDEO DE TANTAN SUR LES ENUMS (+video de let's get rusty ?)
type MapTileMap = Vec<Vec<RoomTileMap>>; 

// pub struct  MapTileMap{
//     table
// }


#[derive(Clone)]

pub struct  RoomTileMap {
    tableRoom : Vec<Vec<Tile>>,

}

impl  RoomTileMap {
    // pub fn createRoom(game : &Game) -> Vec<Vec<Tile>> {
    //     let mut room = vec![vec![Tile::empty(); (game.nbTileHeight) as usize]; (game.nbTileWidth) as usize]; // A CHANGER POUR AVOIR PLUSIEURS ROOM 
    //     for x in 0..(game.nbTileHeight) as usize{
    //         room[x][0]= Tile::wall();
    //         room[x][(game.nbTileWidth -1)  as usize]= Tile::wall();
    //     }

    //     for y in 0..game.nbTileWidth as usize{
    //         room[0][y]= Tile::wall();
    //         room[(game.nbTileHeight -1) as usize][y]= Tile::wall();
    //     }
        
    //     room[7][3] = Tile::wall(); // !!! ATTENTION AU ROOM WIDTH IMPAIR
    //     room[((game.ROOM_HEIGHT/game.TileHeight)/2 -1)as usize][0] = Tile::door();
    //     // room[((ROOM_HEIGHT/TileHeight)/2 +1)as usize ][0] = Tile::empty();
    //     room[((game.ROOM_HEIGHT/game.TileHeight)/2 -1)as usize][(game.ROOM_HEIGHT/game.TileHeight -1)as usize] = Tile::door();
    //     room[0][((game.ROOM_WIDTH/game.TileHeight)/2 -1)as usize] = Tile::door();
    //     room[(game.ROOM_WIDTH/game.TileHeight -1)as usize][((game.ROOM_WIDTH/game.TileHeight)/2 -1)as usize] = Tile::door();
    //     room
        
    // }
    pub fn new() -> Self {
        let mut room = vec![vec![Tile::empty(); (1) as usize]; (1) as usize]; // A CHANGER POUR AVOIR PLUSIEURS ROOM 
        RoomTileMap{tableRoom : room}
    }
    
    pub fn empty(game: &Game ) -> Self {
        let mut room = vec![vec![Tile::empty(); (game.nbTileHeight) as usize]; (game.nbTileWidth) as usize]; // A CHANGER POUR AVOIR PLUSIEURS ROOM 
        for x in 0..(game.nbTileHeight) as usize{
            room[x][0]= Tile::wall();
            room[x][(game.nbTileWidth -1)  as usize]= Tile::wall();
        }

        for y in 0..game.nbTileWidth as usize{
            room[0][y]= Tile::wall();
            room[(game.nbTileHeight -1) as usize][y]= Tile::wall();
        }
        
        room[7][3] = Tile::wall(); // !!! ATTENTION AU ROOM WIDTH IMPAIR
        room[((game.ROOM_HEIGHT/game.TileHeight)/2 -1)as usize][0] = Tile::door();
        // room[((ROOM_HEIGHT/TileHeight)/2 +1)as usize ][0] = Tile::empty();
        room[((game.ROOM_HEIGHT/game.TileHeight)/2 -1)as usize][(game.ROOM_HEIGHT/game.TileHeight -1)as usize] = Tile::door();
        room[0][((game.ROOM_WIDTH/game.TileHeight)/2 -1)as usize] = Tile::door();
        room[(game.ROOM_WIDTH/game.TileHeight -1)as usize][((game.ROOM_WIDTH/game.TileHeight)/2 -1)as usize] = Tile::door();
        RoomTileMap {
            tableRoom : room 
        }
    }
    pub fn centerWall(game: &Game ) -> Self {
        let mut room = vec![vec![Tile::empty(); (game.nbTileHeight) as usize]; (game.nbTileWidth) as usize]; // A CHANGER POUR AVOIR PLUSIEURS ROOM 
        for x in 0..(game.nbTileHeight) as usize{
            room[x][0]= Tile::wall();
            room[x][(game.nbTileWidth -1)  as usize]= Tile::wall();
        }

        for y in 0..game.nbTileWidth as usize{
            room[0][y]= Tile::wall();
            room[(game.nbTileHeight -1) as usize][y]= Tile::wall();
        }
        
        room[7][3] = Tile::wall(); // !!! ATTENTION AU ROOM WIDTH IMPAIR
        // room[((ROOM_HEIGHT/TileHeight)/2 +1)as usize ][0] = Tile::empty();
        room[((game.ROOM_HEIGHT/game.TileHeight)/2 -1)as usize][0] = Tile::door();
        room[((game.ROOM_HEIGHT/game.TileHeight)/2 -1)as usize][(game.ROOM_HEIGHT/game.TileHeight -1)as usize] = Tile::door();
        room[0][((game.ROOM_WIDTH/game.TileHeight)/2 -1)as usize] = Tile::door();
        room[(game.ROOM_WIDTH/game.TileHeight -1)as usize][((game.ROOM_WIDTH/game.TileHeight)/2 -1)as usize] = Tile::door();
        room[14][14] = Tile::wall();
        room[15][14] = Tile::wall();
        room[14][15] = Tile::wall();
        room[15][15] = Tile::wall();
        RoomTileMap {
            tableRoom : room 
        }
    }

    
}


// !!! ENUMS REVOIR LES EXPLICATIONS TANTAN OU LET'S GET RUSTY
#[derive(Clone, Copy)]
pub struct Tile {

    blocked: bool,
    door : bool,
    // block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            door : false
            // block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            door : false
            // block_sight: true,
        }
    }

    pub fn door() -> Self {
        Tile {
            blocked: false,
            door : true,
            // block_sight: true,
        }
    }

    // pub fn getDesignFromTile(&self, game : &Game) -> Vec<Design>  {
    //     let mut listDesign = Vec::new();
    //     for x in 0..(game.ROOM_HEIGHT/game.TileHeight) as usize {
    //         for y in 0..(game.ROOM_WIDTH/game.TileHeight) as usize{
    //             if game.room[x][y].blocked == true{
    //                 let design =  Design::new((y as i32)*game.TileHeight,(x as i32)*game.TileHeight,game.TileHeight as u32,game.TileHeight as u32);
    //                 listDesign.push(design);
    //             }
    //         }
    //     }

    //     listDesign
    // }
}

pub struct Player{
    //Position de la salle actuelle
    design : Design,
    map_x : i32,
    map_y : i32,
    facex : i32,
    facey : i32,
    // inventory : Vec<Item>,
    // activeItem : Vec<Item>,
    // pv : i8,
    // att : i8,
    // def : i8,
    // !!! RASSEMBLER TOUT CA DANS UNE STRUCT STATS ?
    
}

impl Player {

    // fn SayPosition(self)  {
    //     println!("X:{0}, Y:{1}", self.design.x, self.design.y)
    // }

    fn new(x:i32, y:i32, width :u32, height: u32) -> Player{
        let design = Design::new(x,y,height,width,230,210,130);
        // let inventory = Vec::new();
        // let activeItem = Vec::new();
        let player= Player{design : design, map_x : 0, map_y : 0, facex : 0, facey : 0};
        // let player= Player{design : design, map_x : 0, map_y : 0, inventory:inventory, activeItem:activeItem};
        player
    }

    // fn addObjectToInventory(& mut self){
    //     // !!! Inventory inutile atm 
        
    //     self.inventory.push(Item::newBowTest());
    //     self.activeItem.push(Item::newBowTest());

    // }


    fn mouvement(&mut self, game : &Game, x:i32, y:i32){

        let mut blocked = false;
        let mut door = false;
        let positionTile = game.calculPositionTile(self.design.x + x,self.design.y +y, self.design.height, self.design.width);
        println!("{0}!!{1}!!{2}", self.design.x, positionTile.xmin, positionTile.xmax);
        println!("{0}!!{1}", self.map_x, self.map_y);
        self.facex = x ;
        self.facey = y ;
        // let actualMap = game.map[self.map_x as usize][self.map_y as usize];

        // for xi in positionTile.xmin..(positionTile.xmax +1){
        //     if (game.room[(positionTile.ymin) as usize][(xi) as usize].blocked == true)||(game.room[(positionTile.ymax) as usize][(xi) as usize].blocked == true){
        //         blocked = true;
        //     }
        //     else if (game.room[(xi) as usize][(positionTile.xmin) as usize].blocked == true)||(game.room[(xi) as usize][(positionTile.xmax) as usize].door == true) {
        //         door = true;
        //     }
        // }

        // for yi in positionTile.ymin..(positionTile.ymax +1){
        //     if (game.room[(yi) as usize][(positionTile.xmin) as usize].blocked == true)||(game.room[(yi) as usize][(positionTile.xmax) as usize].blocked == true){
        //         blocked = true;
        //     }
        //     else if (game.room[(yi) as usize][(positionTile.xmin) as usize].blocked == true)||(game.room[(yi) as usize][(positionTile.xmax) as usize].door == true) {
        //         door = true;
        //         println!("!!");
        //     }
        // }
        for xi in positionTile.xmin..(positionTile.xmax +1){ // !!! ATTENTION AU || !!! C'EST ICI QUE CE CREE LES PORTES INVISIBLES 
            if (game.map[self.map_x as usize][self.map_y as usize].tableRoom[(positionTile.ymin) as usize][(xi) as usize].blocked == true)||(game.map[self.map_x as usize][self.map_y as usize].tableRoom[(positionTile.ymax) as usize][(xi) as usize].blocked == true){
                blocked = true;
            }
            else if (game.map[self.map_x as usize][self.map_y as usize].tableRoom[(positionTile.ymin) as usize][(xi) as usize].blocked == true)||(game.map[self.map_x as usize][self.map_y as usize].tableRoom[(xi) as usize][(positionTile.ymax) as usize].door == true) {
                door = true;
            }
        }

        for yi in positionTile.ymin..(positionTile.ymax +1){
            if (game.map[self.map_x as usize][self.map_y as usize].tableRoom[(yi) as usize][(positionTile.xmin) as usize].blocked == true)||(game.map[self.map_x as usize][self.map_y as usize].tableRoom[(yi) as usize][(positionTile.xmax) as usize].blocked == true){
                blocked = true;
            }
            else if (game.map[self.map_x as usize][self.map_y as usize].tableRoom[(yi) as usize][(positionTile.xmin) as usize].blocked == true)||(game.map[self.map_x as usize][self.map_y as usize].tableRoom[(yi) as usize][(positionTile.xmax) as usize].door == true) {
                door = true;
                println!("!!");
            }
        }

        if blocked == false { //postiyionTIle.x/y doivent être des u32mais sont des i32 mais les attributs ne peuvent pas être des u32 car dans le rectangle de SDL ont besoin d'être i32 
            if door == false {
                self.design.x += x;
                self.design.y += y;
                println!("x:{0},y:{1}", &x, &y);
            }
            else if door == true{
                println!("PORTE");
                println!("{0}!{1}", self.design.x, self.design.y);
                println!("{0}!{1}!!{2}", positionTile.xmin, positionTile.ymin, game.ROOM_HEIGHT - game.TileHeight -1);
                self.design.x += x;
                self.design.y += y;

                // REMPLACER LES X PAR XMIN ? UNITE!    X = Y
                if 0 < (self.design.x +x) && (self.design.x +x) < game.TileHeight{
                    self.design.x = game.ROOM_HEIGHT - game.TileHeight*2;
                    self.map_x -= 1;
                    println!("x0");
                    println!("{0}|{1}", self.design.x, self.design.y);
                    // println!("{0}", self.map_x);

                }
                else if game.ROOM_HEIGHT > (self.design.x +x) && (self.design.x +x) > (game.ROOM_HEIGHT - game.TileHeight*2 -1) {
                    self.design.x = 0 + game.TileHeight;
                    self.map_x += 1;
                    println!("x1");
                }
                else if 0 < (self.design.y + y) && (self.design.y + y) < game.TileHeight{
                    self.design.y = game.ROOM_WIDTH - game.TileHeight*2;
                    self.map_y -= 1;
                    println!("y0");
                }
                else if game.ROOM_WIDTH > (self.design.y+y) && (self.design.y+y) > (game.ROOM_WIDTH - game.TileHeight*2) {
                    self.design.y = 0 + game.TileHeight ;
                    self.map_y += 1;
                    println!("y1");
                }   
            }        
        }

        else {
            println!("Vous êtes bloqué par quelque chose");
        }

    }

    fn attack(&self, game : &mut Game) -> HitBox{
        // let bow = Item::newBowTest();
        let hitBox = HitBox::new(&self); 
        // game.appendListProject(hitBox);
        // Suivre ça pour ajouter les valeurs du player à HitBox
        println!("Projectile lancé");
        hitBox

    }

    // fn door(x : usize, y : usize){


    // }

}



pub struct HitBox{
    vitessex : i8,
    vitessey : i8,
    design : Design,
    
    
}

impl HitBox{
    pub fn new(player : &Player) -> Self{
        let design = Design::new(player.design.x,player.design.y,1,3,0,0,0);
        let hitbox = HitBox{vitessex : player.facex as i8,vitessey : player.facey as i8, design : design};
        hitbox
    }
}


// enum WeaponKind {
//     Bow{att : i8 },
//     Sword{att : i8 },
// }
// pub struct Weapon {
//     kind : WeaponKind,
//     att : i8, 
//     hitBox : HitBox,
//     Sword{att : i8, projectile: Projectile },
// }

// enum ArmorKind{
//     Armor{},
//     Boots{ms:i8,}
// }

// pub struct  Armor{
//     kind : ArmorKind,
//     def : i8,

// }

// !!! Rajouter une enum intermédiaire pour différencier les armes et le reste COMME AU DESSUS ?  
enum Items {
    Sword{att : i8, hitBox : HitBox },
    Bow{att : i8, hitBox : HitBox},
    Armor{def:i8}

}

pub struct Item {
    // kind : Items,
    att : i8,
    hitBox : HitBox,


}

// impl Item {
//     pub fn newBowTest() -> Self{
//         // let hitbox= HitBox::new();
//         // let item = Item{att:5,hitBox:hitbox}; 
//         // let item = Item{kind : Items::Sword{att:5,hitBox:hitbox}}; 
//         // item
//     }
// }

#[derive(Clone, Copy)]
pub struct Design{
    x:i32,
    y:i32,
    height: u32,
    width: u32,
    colour : [u8; 3],

}

impl Design{
    pub fn new(x:i32,y:i32,height: u32,width: u32, r: u8, g: u8, b:u8) -> Design{
        let design = Design{x:x, y:y, height: height, width: width, colour:[r,g,b]};
        design
    }
    
    pub fn set_texture(self, texture_creator : &TextureCreator<WindowContext>)-> Texture{
        let mut texture = texture_creator
        .create_texture_target(PixelFormatEnum::RGBA8888, self.width, self.height)
        .unwrap();
        texture
    }

}
 
pub struct Position{ //+Taille ? 
    xmax:i32,
    ymax:i32,
    xmin:i32,
    ymin:i32,
}

impl Position{
    pub fn new(xmin:i32,ymin:i32,xmax: i32,ymax: i32) -> Position{
        let position =Position{xmin:xmin, ymin :ymin,xmax:xmax, ymax:ymax};
        position
    }
}

// struct Object{
//     position:Position,
//     block : bool,
// }
// struct Monster{
//     position:Position,

// }


pub struct  Colour {
    R: i8,
    G: i8,
    B :i8
}