#[derive(Debug)]
enum Gender {
    Rock,
    Metal,
    Cumbia,
    Trap,
    Rap
}

enum Selection {
    Playlist,
    Song(Gender)
}


fn play() {

}

fn main() {
    let select: Selection = Selection::Song(Gender::Rap);
    match select {
        Selection::Playlist => (),
        Selection::Song(gender) => println!("Playing {:?}", gender)
    }
}