fn main(){
    let x: i32 = 24;
    let x_ref: &i32 = &x;
    /*
    Shadowing doesn't remove the initial variable so &x still has a value of 24
    This means the space in memory of the first declaration is still occupied after shadowing.
    */
    println!("x:{}, x_ref:{}", x, x_ref); // x:24, x_ref:24

    let x: i32 = 2*x;
    println!("x:{}, x_ref:{}", x, x_ref); // x:48, x_ref:24

    let x: String = String::from("El papu misterioso");
    println!("x:{}, x_ref:{}", x, x_ref); // x:El papu misterioso, x_ref:24
}
