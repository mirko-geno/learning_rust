#[allow(unused_variables)] 
#[allow(dead_code)] 
#[derive(Debug)]
enum Gender {
    Rock,
    Metal,
    Cumbia,
    Trap,
    Rap
}

#[allow(unused_variables)] 
#[allow(dead_code)] 
enum Selection {
    Playlist,
    Song(Gender)
}


fn _play() {}

fn main() {
    let select: Selection = Selection::Song(Gender::Rap);
    match select {
        Selection::Playlist => (),
        Selection::Song(gender) => println!("Playing {:?}", gender)
    }
}