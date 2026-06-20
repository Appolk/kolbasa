use std::time::Duration;

fn main() {
    const MP3_BYTES:&[u8]= include_bytes!("kolbasa.mp3");
    

    let handle = rodio::DeviceSinkBuilder::open_default_sink()
        .expect("open_default_sink;");
    
    let mixer = &handle.mixer();

    let player = rodio::Player::connect_new(mixer);

    let source = rodio::Decoder::try_from(std::io::Cursor::new(MP3_BYTES))
        .expect("try_from;");

    

    
    mixer.add(source);

    std::thread::sleep(Duration::from_secs(1 * 60 + 4)); // 4 min

}
