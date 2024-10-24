fn modify_string(string: &mut String){
    string.push_str("banana");
}


fn main(){
    // This is horrible
    let mut stri: String = String::from("El pepe");
    let mut _mut_stri_ref: &mut String = &mut stri;
    // If we passed stri to println!, mut_stri_ref would be invalid due to the creation of an inmutable reference
    println!("String is: {_mut_stri_ref}");
    modify_string(_mut_stri_ref);
    println!("String after moficiation is {_mut_stri_ref}");

    
    //Instead it could be done in this way
    let mut stri: String = String::from("El pepe");
    println!("String is {stri}"); // Use of inmutable ref
    modify_string(&mut stri); 
    println!("String after moficiation is {stri}");
}