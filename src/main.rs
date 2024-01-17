fn add(num_one: i32, num_two: i32) -> i32{
    return num_one + num_two; 
}

fn free_ship(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}
fn main() {
    // variables

    let my_name = "Shafi";
    println!("{}", my_name);

    // Operators
    let add_two_numbers = add(2, 4);
    println!("{:?}", add_two_numbers);

    // Conditional statement

    let total_purchase = free_ship(22, 30);
    if total_purchase > 50 {
        println!("You qualify for free shipping!");

    }
    else if total_purchase > 25{
        println!("If you add more items, you can qualify for free shipping");

    }
    else {
        println!("Please Add more items");

    }

}
