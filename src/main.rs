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

    // Arrays
    let items = [1, 2, 3, 4, 5]; 
    println!("{:?}", items);

    let vector_items = vec![1, 2, 3, 4, 5];
    let mut vector_items_2 = Vec::new();
    vector_items_2.push(1);
    vector_items_2.push(2);
    vector_items_2.push(3);
    vector_items_2.push(4);
    vector_items_2.push(5);
    vector_items_2.push(6);

    println!("{:?}", vector_items);
    println!("{:?}", vector_items_2);


}
