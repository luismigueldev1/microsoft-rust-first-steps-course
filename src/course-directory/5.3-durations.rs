// fn main() {
//     let x;
//     {
//         let y = 42;
//         x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
//     }
//     println!("x: {}", x); // `x` refers to `y` but `y has been dropped!
// }

// fn main() {
//     let magic1 = String::from("abracadabra!");
//     let magic2 = String::from("shazam!");

//     let result = longest_word(&magic1, &magic2);
//     println!("The longest magic word is {}", result);
// }

// fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let magic1 = String::from("abracadabra!");
//     let result;
//     {
//         let magic2 = String::from("shazam!");
//         result = longest_word(&magic1, &magic2);
//     }
//     println!("The longest magic word is {}", result);
// }

// fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn erase(_: &String) {}

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);

    erase(&text);

    println!("{:?}", fox);
    println!("{:?}", dog);
}
