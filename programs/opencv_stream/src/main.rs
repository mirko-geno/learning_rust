use opencv;
use opencv_stream::Stream;


fn main() {
    let mut stream = Stream::new(7);
    while opencv::highgui::wait_key(1).unwrap() != 27 { // Exit on 'Esc'
        stream.update();
        stream.show();
    } 
}