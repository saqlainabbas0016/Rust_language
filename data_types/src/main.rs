fn main() {
    println!("Data type in  rust");

    // scalar data types
    // integer , string , boolean , floting  , char ,
    
    // lenght (8bit , 16bit , 32bit , 64bit , 128bit , arch isize , usize)
    // signed (i8 , i16 , i32 , i64 , i128 , isize)
    // unsigned (u8 , u16 , u32 , u64 , u128 , usize)
    // floating (f32 , f64)



    // scalar data types = only one value at a time

    let no = 21;
    println!("{}",no);

    let is_male = true;
    println!("{}",is_male);

    let chart = "abc";
    println!("{}",chart);

    let dec = 70.999;
    println!("{}",dec);

    // compound data types = multiple values at a time
    // array , tuples , dictionary

    // tuples
    let tup = (1,2,3,4,5);
    println!("{:?}",tup);
    panic!("{}",tup.0);

}
