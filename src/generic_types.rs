// struct Point<T, U> {
//     x: T,
//     y: U,
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn main() {
    //
    // let number_list = vec![10, 2, 30, 40];
    // let largest = find_largest(number_list);
    // let char_list = vec!['a', 'b', 'c'];
    // let largest_char = find_largest(char_list);
    // println!("{}", largest);
    // println!("{}", largest_char);

    let p = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    // let p3 = Point { x: 5, y: 10.0 };
}

// fn find_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }
