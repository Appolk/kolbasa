

use std::{io::{ Write, stdout}, thread, time::{Duration, Instant}};

use rodio::Source;


fn main() {
    const MP3_BYTES:&[u8]= include_bytes!("kolbasa.mp3");
    

    let mut handle = rodio::DeviceSinkBuilder::open_default_sink()
        .expect("open_default_sink;");

    handle.log_on_drop(false);


    let mixer = &handle.mixer();

    let player = rodio::Player::connect_new(mixer);

    let source = rodio::Decoder::try_from(std::io::Cursor::new(MP3_BYTES))
        .expect("try_from;");
    
    let mp3_duration = source.total_duration();
    let total_mss = mp3_duration.unwrap().as_millis();
    
    
    mixer.add(source);
    let start_time = Instant::now();


    const WIDTH:usize = 100;
    loop {
        let pos = start_time.elapsed();

     

        if pos >= mp3_duration.unwrap(){
            println!("\nexit");
            break;
        }
       
       let progress = (pos.as_millis() as f64 / total_mss as f64).min(1.0);

       let n_dashes = (progress * WIDTH as f64) as usize; 
       let n_spaces = WIDTH - n_dashes;

       let dashes = "-".repeat(n_dashes);
       let spaces = " ".repeat(n_spaces);

       print!("\r[{}{}]",dashes,spaces);
       let _ = stdout().flush();
        thread::sleep(Duration::from_millis(1));
     
      
    }
    

    
  

    
    player.sleep_until_end();
}
