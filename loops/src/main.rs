fn main() {
    println!("Loops ");

    first_loop() ;
 secod_loop() ;
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
    
// while loop 


fn secod_loop(){
    
    let a = [ 10,20,30,40,50,60];
    
    let mut index = 0;
    
    while index < 7 {

        println!("the value is {}", a[index]) ;

        index += 1 ;
    }

}

// for loop 

fn third_loop (){
    
}
