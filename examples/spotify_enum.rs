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


fn main() {
    let select: Selection = Selection::Song(Gender::Rap);
    match  select{
        
    }

}