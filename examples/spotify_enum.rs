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
    Song(Gender) {
        println!("{:?}", )
    }
}


fn play() {

}

fn main() {
    let select: Selection = Selection::Song(Gender::Rap);
    match select {
        Selection::Playlist => 0,
        Selection::Song(gender) => {
            println!("Playing {:?}", gender);
            1
        }
        
    }

}