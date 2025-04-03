fn main() {
    println!("Loops ");

    first_loop() ;

}

//types of loops 

// loop 
// while
// for 
 

fn first_loop() {
    let mut x = 0 ;
    loop {

    x += 1 ;
    println!("x = {}", x);
     if x == 5 {
     println!(" we did it ");
     break;
    }
}

}