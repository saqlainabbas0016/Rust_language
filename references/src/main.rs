fn main() {
    println!("references");
    first();
    second();
}

fn first() {
    let x = 5;
    let y = &x; // y is a reference to x
    let z = &x;
    
    println!("x: {}, y: {} , z : {}", x, y , z);
}
 
fn second() {
    let mut x = 5;
    let z = &mut x; // mutable reference
    

    *z += 1; // dereference z to change the value of x
    println!("z: {}", z);
}