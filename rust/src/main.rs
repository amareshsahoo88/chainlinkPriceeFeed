#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(non_snake_case)]


fn main(){
    println!("Hello world");

    let mut some_data: bool = true;

    some_data = false;

    println!("{}" , {some_data});

    let some_data: i8 = 10;
    let dougs_test = some_data +120 ;
    println!("{}" , dougs_test );


    let some_data: u128 = 10;

    println!("Min i128 is {}" , std::u128::MIN);
    println!("Max i128 is {}" , std::u128::MAX);


    let some_data: f32 = 10.23;
    println!("{}" , some_data );

    let some_char: char = 'a';


}