fn main() {
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // fn divide(num: f64, den: f64) -> Option<f64> {
    //     if den == 0.0 {
    //         None
    //     } else {
    //         Some(num / den)
    //     }
    // }

    // let result = divide(10.0, 1.0);
    // match result {
    //     Some(x) => println!("Result: {}", x),
    //     None => println!("Error"),
    // };

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    fn divide(num: f64, den: f64) -> Result<f64, String> {
        if den == 0.0 {
            Err("Cannot divide by Zero".to_string())
        } else {
            Ok(num / den)
        }
    }
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result is: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
