fn main() {
    println!("Ownership tutorial");


    first();
    second();
}

fn first(){
    let x = 90;
    println!("{}", x) ;

    let y = x;
    println!( "{}", y) ;
}



fn second(){
    let a = String::from("gilgit, danyour");

    let b = a.clone();
   
    println!("{}", b);
}




