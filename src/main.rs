// TO DO :
//     1-Voir le book "https://gameprogrammingpatterns.com/book" ! Implémenter des threads ? Choisir un pattern de programmation pour faire du clean
//     2-Implémenter une game loop plus propre (treads )


//     3-créer des obstaclesV
//     4-implémenter la room (qui deviendra une TileMap) et faire un algo qui la créer intelligementV/2
//  4.5 - Changement de carte V
//     5-monstres qui tuent quand on les touchent   AFAIRE
//     6-IA pour les monstres   V/2
//     7-Des projectiles V
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
// Verifier les algo ?

// Coder les déplacement des monstres par rapport au mur
// Si monstre touche player att
// Meilleure algo de monstres pour vérifier tout ce qui est autour de lui ? 
// Une nouvelle map au dessus des tiles pour ne pas avoir à récupérer les valeurs de chaque objets mais celle de la map (dans game)
// CODER UN? COUP D'EPEE



use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Texture, TextureCreator};
use sdl2::rect::*;
use sdl2::video::WindowContext;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::image::{self, LoadTexture, InitFlag};
use std::string;
use std::time::Duration;


// mod object;
// use object::Player;


fn main() -> Result<(), String> {
  
    let mut game = Game::new(512,512);
    let mut player = Player::new(150, 100, 16, 16);
    // player.addObjectToInventory();
    // println!("{}",player.inventory.len());
    let mut playernumdeux = Player::new( 130, 130, 32, 32);
    
    
    
    
    
    
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);
    
    
    let window = video_subsystem.window("test Roguelike",game.ROOM_WIDTH as u32 , game.ROOM_HEIGHT as u32)
    .position_centered()
    .build()
    .expect("123");
    
    let mut canvas  = window.into_canvas()
                                        .present_vsync()
                                        .build()
                                        .expect("msg");
    let creator = canvas.texture_creator();
    
    let mut event_pump = sdl_context.event_pump()?;
    let texturePlayer = creator.load_texture("assets/bardo.png")?;
    
    let mut listOfProjectile : Vec<HitBox> = Vec::new();
    let mut listOfMonster : Vec<Monster> = Vec::new();
    // let mut listOfPlayer : Vec<Player> = Vec::new();
    // listOfPlayer.push(player);

    // let listOfProjectile_iter = listOfProjectile.iter_mut();
    // let listOfMonster_iter =listOfMonster.iter();
    // listOfProjectile;
    let mut monster1 = Monster::new(60,60);
    let mut monster2 = Monster::new(60,80);
    let mut monster3 = Monster::new(80,60);
    let mut monster4 = Monster::new(160,60);
    let mut monster5 = Monster::new(60,160);
    listOfMonster.push(monster1);
    listOfMonster.push(monster2);
    listOfMonster.push(monster3);
    listOfMonster.push(monster4);
    listOfMonster.push(monster5);
    
    
    
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

        // let ListObjects = [&player, &playernumdeux];



        for monster in &mut listOfMonster{
            if monster.map_x == player.map_x && monster.map_y == player.map_y{
                monster.calculDistance(&player);

            }

        }


        let mut listDesignTile = game.getDesignFromTile(&player);
        // listDesignTile.push(player.design);
        // let mut texture = player.design.set_texture(&creator);
        let rectag = Rect::new(0,0, 26,36);
        let rect = Some(Rect::new(player.design.x,player.design.y, 26, 36));
        canvas.set_draw_color(Color::RGB(50, 50, 70));
        canvas.copy(&texturePlayer, None, rect)?;



        // listDesignTile.push(monster.design);
        // let mut listOfElementToRemove : Vec<i8> = Vec::new() ;
        // let mut listOfMonsterToRemove : Vec<i8> = Vec::new() ;
        // println!("{}", listOfProjectile.len());
        // let mut monsters : Vec<Monster> = Vec::new();
        // monsters.push(monster);

        //RENAME TILE EN OBJECT
        // !!! LA FENETRE RAME LORSQUE JE LUI PASSE UNE IMAGE PAR OBJECT (LOGIQUE ?) 
        for tile in 0..listDesignTile.len(){

            let mut texture = listDesignTile[tile].set_textureColor(&creator);
            // let mut texture = creator.load_texture(&listDesignTile[tile].img)?;
            // let mut texture = creator.load_texture("assets/bardo.png")?;


            let rect = Some(Rect::new(listDesignTile[tile].x,listDesignTile[tile].y, listDesignTile[tile].width, listDesignTile[tile].height));
            // canvas
            //     .with_texture_canvas(&mut texture, |texture_canvas|{
            //         texture_canvas.clear();
            //         // texture_canvas.set_draw_color(Color::RGB(listDesignTile[tile].colour[0],listDesignTile[tile].colour[1], listDesignTile[tile].colour[2]));
            //         texture_canvas.fill_rect(Rect::new(0, 0, listDesignTile[tile].width, listDesignTile[tile].width)).expect("msg"); // !!!REVOIR CA CARRE DE 400 POUR TEXTURE DE 16 
    
            // }).expect("123");
            canvas.set_draw_color(Color::RGB(50, 50, 70));
            canvas.copy(&mut texture, None, rect)?;

            // if tile == (listDesignTile.len() -1){
            //     player.design = listDesignTile[tile];
                
            // }
        }
        // player.design = listDesignTile.remove(listDesignTile.len()-1);
        
        //Transformer listObjects en Design (c'est déjà des joueurs)

        // for object in ListObjects{
        //     let mut texture = object.design.set_texture(&creator);
        //     let rect = Some(Rect::new(object.design.x,object.design.y, object.design.width, object.design.height));
        //     canvas
        //         .with_texture_canvas(&mut texture, |texture_canvas|{
        //             texture_canvas.clear();
        //             texture_canvas.set_draw_color(Color::RGB(80,175, 230));
        //             texture_canvas.fill_rect(Rect::new(0, 0, 400, 300)).unwrap();
    
        //     }).unwrap();
        //     canvas.set_draw_color(Color::RGB(50, 50, 70));
        //     canvas.copy(&mut texture, None, rect).unwrap();
    

        // }
        
        // METTRE PROJECTCALCUL AILLEURS
        // !!! ATTENTION A CE I EN ITERATION C'EST BIZARRE
        // let mut i = 0;
        'projectilecalcul : for object in (0..listOfProjectile.len()).rev(){
            // println!("{}", listOfProjectile.len());
            let mut blocked = false;
            let mut door = false;
            let mut isAlive = true;

            // println!("X : {0} § Y : {1}", object.design.x, object.design.y);
            if listOfProjectile[object].map_x == player.map_x && listOfProjectile[object].map_y == player.map_y{
                
                let positionTile = game.calculPositionTile(listOfProjectile[object].design.x + listOfProjectile[object].vitessex as i32, listOfProjectile[object].design.y + listOfProjectile[object].vitessey as i32, listOfProjectile[object].design.height, listOfProjectile[object].design.width);
                for xi in positionTile.xmin..(positionTile.xmax +1){ // !!! ATTENTION AU || !!! C'EST ICI QUE CE CREE LES PORTES INVISIBLES 
                    if (game.map[player.map_x as usize][player.map_y as usize].tableRoom[(positionTile.ymin) as usize][(xi) as usize].blocked == true)||(game.map[player.map_x as usize][player.map_y as usize].tableRoom[(positionTile.ymax) as usize][(xi) as usize].blocked == true){
                        blocked = true;
                        // println!("1");
                    }
                    else if (game.map[player.map_x as usize][player.map_y as usize].tableRoom[(positionTile.ymin) as usize][(xi) as usize].blocked == true)||(game.map[player.map_x as usize][player.map_y as usize].tableRoom[(xi) as usize][(positionTile.ymax) as usize].door == true) {
                        door = true;
                        // println!("2");
    
                    }
                }
    
                for yi in positionTile.ymin..(positionTile.ymax +1){
                    if (game.map[player.map_x as usize][player.map_y as usize].tableRoom[(yi) as usize][(positionTile.xmin) as usize].blocked == true)||(game.map[player.map_x as usize][player.map_y as usize].tableRoom[(yi) as usize][(positionTile.xmax) as usize].blocked == true){
                        blocked = true;
                        // println!("3");
    
                    }
                    else if (game.map[player.map_x as usize][player.map_y as usize].tableRoom[(yi) as usize][(positionTile.xmin) as usize].blocked == true)||(game.map[player.map_x as usize][player.map_y as usize].tableRoom[(yi) as usize][(positionTile.xmax) as usize].door == true) {
                        door = true;
                        // println!("4");
                    }
                }
                if blocked == true || door == true {
                    listOfProjectile.remove(object);
                    // listOfElementToRemove.push(i);

                }
                else {
                    'monsterloop: for monster in (0..listOfMonster.len()).rev(){

                        // let monsterTile = game.calculPositionTile(monster.design.x, monster.design.y, monster.design.height, monster.design.width);
                        if (listOfMonster[monster].design.x <= listOfProjectile[object].design.x && listOfProjectile[object].design.x <= (listOfMonster[monster].design.x + listOfMonster[monster].design.width as i32)) && (listOfMonster[monster].design.y <= listOfProjectile[object].design.y && listOfProjectile[object].design.y <= (listOfMonster[monster].design.y + listOfMonster[monster].design.height as i32)){
                            
                            listOfMonster[monster].pv -= listOfProjectile[object].att;
                            println!("!");
                            listOfProjectile.remove(object);
                            isAlive = false;
                            
                            if listOfMonster[monster].pv <= 0{
                                listOfMonster.remove(monster);
                                println!("||{}||", monster );
                            
                            }
                            break 'monsterloop;
                        }
                    }
                    if isAlive {
                        listOfProjectile[object].design.x += listOfProjectile[object].vitessex as i32; 
                        listOfProjectile[object].design.y += listOfProjectile[object].vitessey as i32; 
                    }
                    // println!("6");
                }

                // listOfProjectile.remove(i);
                // i+=1;
                // continue 'projectcalcul;
            }

            else {                    
                listOfProjectile.remove(object);

                // listOfElementToRemove.push(i);

            }


            
            // i+=1;
        }

        // for index in listOfElementToRemove{
        //     println!("{}", index);
        //     listOfProjectile.remove(index as usize);
        // }

        // for index in listOfMonsterToRemove{
        //     listOfMonster.remove(index as usize);
        // }

        for object in &listOfProjectile{
            let mut texture = object.design.set_textureColor(&creator);
            let rect = Some(Rect::new(object.design.x,object.design.y, object.design.width, object.design.height));
            canvas
                .with_texture_canvas(&mut texture, |texture_canvas|{
                    texture_canvas.clear();
                    texture_canvas.set_draw_color(Color::RGB(object.design.colour[0], object.design.colour[1], object.design.colour[2]));
                    texture_canvas.fill_rect(Rect::new(0, 0, object.design.width, object.design.height)).expect("msg");
    
            }).expect("msg");
            canvas.set_draw_color(Color::RGB(50,50, 70));
            canvas.copy(&mut texture, None, rect)?;
    

        }
        
        // !!! CHECK LES FONCTIONS DESSINANT PLUSIEURS RECT EN UN APPEL
        for object in &listOfMonster{
            if object.map_x == player.map_x && object.map_y == player.map_y{

                let mut texture = object.design.set_textureColor(&creator);
                let rect = Some(Rect::new(object.design.x,object.design.y, object.design.width, object.design.height));
                canvas
                    .with_texture_canvas(&mut texture, |texture_canvas|{
                        texture_canvas.clear();
                        texture_canvas.set_draw_color(Color::RGB(object.design.colour[0], object.design.colour[1], object.design.colour[2]));
                        texture_canvas.fill_rect(Rect::new(0, 0, object.design.width, object.design.height)).expect("msg");
        
                }).expect("msg");
                canvas.set_draw_color(Color::RGB(50,50,70));
                canvas.copy(&mut texture, None, rect)?;
            }
    

        }

        
        canvas.present();
        canvas.clear();


        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
    
    
    }   
    Ok(())
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
    listOfProjectile : Vec<HitBox>,
    changeMap : bool,

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


        let game = Game {map : map,  TileHeight : TileHeight, ROOM_HEIGHT : ROOM_HEIGHT, ROOM_WIDTH : ROOM_WIDTH, nbTileHeight : nbTileHeight ,nbTileWidth: nbTileWidth, MAP_WIDTH :MAP_WIDTH, MAP_HEIGHT :MAP_HEIGHT, listOfProjectile:listOfProjectile, changeMap:true};
        let mut map = vec![vec![RoomTileMap::empty(&game); (MAP_HEIGHT) as usize]; (MAP_WIDTH) as usize];
        map[0][1] = RoomTileMap::centerWall(&game);  
        map[1][1] = RoomTileMap::centerWall(&game);  
        map[1][2] = RoomTileMap::centerWall(&game);  
        let mut listOfProjectile = Vec::new(); 
        let game = Game {map : map,  TileHeight : TileHeight, ROOM_HEIGHT : ROOM_HEIGHT, ROOM_WIDTH : ROOM_WIDTH, nbTileHeight : nbTileHeight ,nbTileWidth: nbTileWidth, MAP_HEIGHT : MAP_HEIGHT, MAP_WIDTH : MAP_WIDTH, listOfProjectile:listOfProjectile,changeMap:true} ;

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
        let actualMap = &self.map[player.map_x as usize][player.map_y as usize];
        for x in 0..(self.ROOM_HEIGHT/self.TileHeight) as usize {
            for y in 0..(self.ROOM_WIDTH/self.TileHeight) as usize{
                if actualMap.tableRoom[x][y].blocked == true{
                    let design =  Design::new((y as i32)*self.TileHeight,(x as i32)*self.TileHeight,self.TileHeight as u32,self.TileHeight as u32,65,20,65, "assets/darkdimension.png".to_string(),128,64);
                    listDesign.push(design);
                }
                if actualMap.tableRoom[x][y].door == true{
                    let design =  Design::new((y as i32)*self.TileHeight,(x as i32)*self.TileHeight,self.TileHeight as u32,self.TileHeight as u32,160,160,160, "assets/darkdimension.png".to_string(),32,16);
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
        let design = Design::new(x,y,height,width,230,210,130, "assets/bardo.png".to_string(),0,0);
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
        // println!("{0}!!{1}!!{2}", self.design.x, positionTile.xmin, positionTile.xmax);
        // println!("{0}!!{1}", self.map_x, self.map_y);
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
                // println!("!!");
            }
        }

        if blocked == false { //postiyionTIle.x/y doivent être des u32mais sont des i32 mais les attributs ne peuvent pas être des u32 car dans le rectangle de SDL ont besoin d'être i32 
            if door == false {
                self.design.x += x;
                self.design.y += y;
                // println!("x:{0},y:{1}", &x, &y);
            }
            else if door == true{
                // println!("PORTE");
                // println!("{0}!{1}", self.design.x, self.design.y);
                // println!("{0}!{1}!!{2}", positionTile.xmin, positionTile.ymin, game.ROOM_HEIGHT - game.TileHeight -1);
                self.design.x += x;
                self.design.y += y;

                // REMPLACER LES X PAR XMIN ? UNITE!    X = Y
                if 0 < (self.design.x +x) && (self.design.x +x) < game.TileHeight{
                    self.design.x = game.ROOM_HEIGHT - game.TileHeight*2;
                    self.map_x -= 1;
                    // game.changeMap = true;
                    // println!("x0");
                    // println!("{0}|{1}", self.design.x, self.design.y);
                    // println!("{0}", self.map_x);

                }
                else if game.ROOM_HEIGHT >= (self.design.x +x) && (self.design.x +x) >= (game.ROOM_HEIGHT - game.TileHeight*2 -1) {
                    self.design.x = 0 + game.TileHeight;
                    self.map_x += 1;
                    // game.changeMap = true;

                    // println!("x1");
                }
                else if 0 <= (self.design.y + y) && (self.design.y + y) <= game.TileHeight{
                    self.design.y = game.ROOM_WIDTH - game.TileHeight*2;
                    self.map_y -= 1;
                    // game.changeMap = true;

                    // println!("y0");
                }
                else if game.ROOM_WIDTH >= (self.design.y+y) && (self.design.y+y) >= (game.ROOM_WIDTH - game.TileHeight*2) {
                    self.design.y = 0 + game.TileHeight ;
                    self.map_y += 1;
                    // game.changeMap = true;

                    // println!("y1");
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

pub struct Monster{
    design : Design,
    map_x : i32,
    map_y : i32,
    pv : i8,
    distance_secu : i16
    // !!!  STRUCT STATS ?
    
}

impl Monster{
    pub fn new(x : i32, y : i32) -> Self{
        let design = Design::new(x,y,16,16,128,84,176, "".to_string(),0,0);
        Monster { design: design, map_x: 0, map_y: 0, pv : 10, distance_secu : 64 }
    }

    pub fn mouvement(&mut self){
        // algo de mouvement aléatoire 
    }

    pub fn calculDistance(&mut self,player : &Player){
        let x = self.design.x - player.design.x;
        let y  = self.design.y - player.design.y;
        let distCarre : f32 = (x*x+y*y) as f32;
        let dist = (distCarre.sqrt()) as i16;
        // println!("{0}!!!{1}",player.design.x, self.design.x);
        
        if dist <= self.distance_secu && !((((player.design.x) <= self.design.x && self.design.x <= (player.design.x + player.design.width as i32)) || ((player.design.x) <= (self.design.x + self.design.width as i32 )  && (self.design.x + self.design.width as i32) <= (player.design.x + player.design.width as i32))) && (((player.design.y) <= self.design.y && self.design.y <= (player.design.y + player.design.height as i32)) || ((player.design.y) <= (self.design.y + self.design.height as i32 )  && (self.design.y + self.design.height as i32) <= (player.design.y + player.design.height as i32))) )   {
            if x*x<=y*y {
                if y<0 {
                    self.design.y +=1;
                    // println!("0");
                
                }
                else {
                    self.design.y -=1;
                    // println!("1");
                    
                }
            }
            else if y*y<x*x {
                if x<0 {
                    self.design.x +=1;
                    // println!("2");
                    
                }
                else {
                    self.design.x -=1;
                    // println!("3");
                    
                }
            }
        }
        else if (((player.design.x) <= self.design.x && self.design.x <= (player.design.x + player.design.width as i32)) || ((player.design.x) <= (self.design.x + self.design.width as i32 )  && (self.design.x + self.design.width as i32) <= (player.design.x + player.design.width as i32))) && (((player.design.y) <= self.design.y && self.design.y <= (player.design.y + player.design.height as i32)) || ((player.design.y) <= (self.design.y + self.design.height as i32 )  && (self.design.y + self.design.height as i32) <= (player.design.y + player.design.height as i32))) {
            if x<0 && y<0{
                self.design.x -= 5;
                self.design.y -= 5;

            }

            else if x>0 && y>0{
                self.design.x += 5;
                self.design.y += 5;
            }
            
            else if x<0 && y>0{
                self.design.x -= 5;
                self.design.y += 5;

            }
            
            else if x>0 && y<0{
                self.design.x += 5;
                self.design.y -= 5;

            }            
            
            else if x<0{
                self.design.x -= 9;
            }
            else if x>1 {
                self.design.x += 9;
                
            }
            else if y<0 {
                self.design.y -= 9;
                
            }
            else if y>0 {
                self.design.y += 9;
                
            }
        }

    }
}

pub struct HitBox{
    map_x : i32,
    map_y : i32,
    vitessex : i8,
    vitessey : i8,
    design : Design,
    att : i8,
    
    
}

impl HitBox{
    pub fn new(player : &Player) -> Self{
        let design = Design::new(player.design.x,player.design.y,1,3,255,249,173, "".to_string(),0,0);
        let hitbox = HitBox{vitessex : player.facex as i8,vitessey : player.facey as i8, design : design, att : 5, map_x : player.map_x, map_y : player.map_y};
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

#[derive(Clone)]
pub struct Design{
    x:i32,
    y:i32,
    height: u32,
    width: u32,
    colour : [u8; 3],
    img : String,
    positionInSpriteX : i32, 
    positionInSpriteY : i32,

}

impl Design{
    pub fn new(x:i32,y:i32,height: u32,width: u32, r: u8, g: u8, b:u8, img : String,positionInSpriteX:i32, positionInSpriteY: i32) -> Design{
        let design = Design{x:x, y:y, height: height, width: width, colour:[r,g,b], img:img, positionInSpriteX:positionInSpriteX, positionInSpriteY:positionInSpriteY};
        design
    }
    
    pub fn set_texture<'a>(&'a self, texture_creator : &'a TextureCreator<WindowContext>)-> Texture<'a>{
        let mut texture = texture_creator.load_texture(&self.img).expect("azer");
        // .create_texture_target(PixelFormatEnum::RGBA8888, self.width, self.height)
        texture

    }
    pub fn set_textureColor<'a>(&'a self, texture_creator : &'a TextureCreator<WindowContext>)-> Texture<'a>{
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