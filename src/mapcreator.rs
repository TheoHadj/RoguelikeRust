fn MapCreator()-> Result<(), String>{
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
}