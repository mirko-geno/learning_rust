fn main() {
    let vector = vec![1, 2, 3];

    let result: Vec<_> = vector.iter().map(|x| x+1).collect();
    println!("The result of {:?} sum is {:?}", vector, result)
}
