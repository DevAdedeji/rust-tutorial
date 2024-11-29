fn main() {
    // hello_world();
    // tell_height(32);
    // human_id("Adedeji", 32, 13.5);
    // let x = {
    //     let price = 5;
    //     let qty: i32 = 10;
    //     price * qty
    // };
    // let x = add(32, 64);
    // println!("Result is, {}", x);
    // println!("Value from function 'add' is: {}.", add(4, 5));

    // Calling the BMI function
    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is: {:.3}", bmi);
}

// fn hello_world() {
//     println!("Hello Rust 🦀");
// }

// fn tell_height(height: i32) {
//     println!("My height is: {}", height);
// }

// fn human_id(name: &str, age: u32, height: f32) {
//     println!(
//         "My name is {}, I am {} years old, and my height is {} cm.",
//         name, age, height
//     );
// }

// Function returning values
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// BMI = weight(kg)/height(m)^2

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
