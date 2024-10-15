fn main(){
    // Scalar variables, due to being storaged in the stack, can be copied easily.
    let scalar_variable: u64 = 6322625;
    let scalar_copy: u64 = scalar_variable;
    println!("Scalar variable: {}, Scalar copy: {}", scalar_variable, scalar_copy); //Scalar variable: 6322625, Scalar copy: 6322625

    /* Variables that contain a pointer to content allocated in the heap can't be copied
    to avoid a double free situation when Rust calls the drop function at the end of a scope.
    For this reason these kind of variables are moved when assigned to other one.*/


    let allocated_variable: String = String::from("I love melons");
    let allocated_copy: String = allocated_variable;
    /* allocated_variable was moved to allocated copy so trying to use it
    will end up in the following error: borrow of moved value: `allocated_variable`
    println!("Allocated variable: {}, Allocated copy: {}", allocated_variable, allocated_copy);*/

    // To fix it a deep copy must be done using the clone method, though it isn't efficient
    let allocated_variable: String = String::from("I love melons");
    let allocated_copy: String = allocated_variable.clone();
    println!("Allocated variable: {}, Allocated copy: {}", allocated_variable, allocated_copy); // Allocated variable: I love melons, Allocated copy: I love melons
}