// fn main() {
//     //let greeting = String::from("hello");
//     // // We borrow `greeting` but the string data is still owned by `greeting`
//     //let greeting_reference = &greeting;
//     // // We can still use `greeting`
//     //println!("Greeting: {}", greeting);

//     let mut hello: String = String::from("Hello");
//     change(&mut hello);
//     println!("{}", hello);
// }

// fn change(str: &mut String) {
//     str.push_str(", world!")
// }

// Referencias en funciones
// fn print_greeting(message: &String) {
//     println!("Greeting: {}", message);
//   }

//   fn main() {
//     let greeting = String::from("Hello");
//     print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
//     print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again
//   }

fn main() {
    let mut value = String::from("hello");

    //let ref1 = &mut value;
    let ref2 = &mut value;

    println!("{}, {}", 1, ref2);
}
