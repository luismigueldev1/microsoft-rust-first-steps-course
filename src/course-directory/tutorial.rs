fn main() {
    // println!("Hello, world!");
    // // todo!("It is a todo");
    // println!(
    //     "the first letter  of the alphabet is: {} and the last is: {}",
    //     "A", "Z"
    // );

    //mutability
    // let mut a_number: i32 = 10;
    // println!("The current funds are: {}", a_number);
    // a_number = 2;
    // println!("Don't spend money in shit! you have now: {}", a_number);

    // variables usage
    // let shadow_num: i32 = 5;
    // let shadow_num: i32 = shadow_num + 5;
    // println!("The number is {}.", shadow_num);
    // let shadow_num = shadow_num * 2;
    // println!("The number is {}.", shadow_num);

    // let is_bigger: bool = 1 > 4;
    // println!("Is 1 > 4? {}", is_bigger);
    // let first_name: &str = "Luis";
    // println!("My first name is : {}", first_name);

    // let char_1: char = 'S';
    // let char_2: char = 'f';
    // let smiley_face: char = '😀';
    // let string_1: &str = "miley ";
    // let string_2: &str = "ace";
    // println!(
    //     "{} is a {}{}{}{}",
    //     smiley_face, char_1, string_1, char_2, string_2
    // );

    // let tuple_example: (char, i32, bool) = ('E', 5i32, true);
    // println!(
    //     "Is: '{}' the {}th letter of the alphabet? {}",
    //     tuple_example.0, tuple_example.1, tuple_example.2
    // );
    // Classic struct with named fields
    // struct Student {
    //     name: String,
    //     level: u8,
    //     remote: bool,
    // }
    // Tuple struct with data types only
    //struct Grades(char, char, char, char, f32);
    // Unit struct
    // struct Unit;
    // Instantiate classic struct, specify fields in random order, or in specified order
    // let user_1 = Student {
    //     name: "Constance Sharma".to_string(), //diferent way to parse to String
    //     remote: true,
    //     level: 2,
    // };
    // let user_2 = Student {
    //     name: String::from("Dyson Tan"), // tutorial wave to convert to string.
    //     level: 5,
    //     remote: false,
    // };
    // // Instantiate tuple structs, pass values in same order as types defined
    // let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    // let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    // println!(
    //     "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
    //     user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4
    // );
    // println!(
    //     "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
    //     user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4
    // );

    // enum WebEvent {
    //     // An enum variant can be like a unit struct without fields or data types
    //     WELoad,
    //     // An enum variant can be like a tuple struct with data types but no named fields
    //     WEKeys(String, char),
    //     // An enum variant can be like a classic struct with named fields and their data types
    //     WEClick { x: i64, y: i64 },
    // }

    //Enum
    // #[derive(Debug)]
    // struct KeyPress(String, char);
    // #[derive(Debug)]
    // struct MouseClick {
    //     x: i64,
    //     y: i64,
    // }

    // #[derive(Debug)]
    // enum WebEvent {
    //     WELoad(bool),
    //     WEClick(MouseClick),
    //     WEKeys(KeyPress),
    // }

    // let click: MouseClick = MouseClick { x: 100, y: 250 };
    // println!("Mouse click location: {}, {}", click.x, click.y);

    // let keys: KeyPress = KeyPress(String::from("Ctrl+"), 'N');
    // println!("\nKeys pressed: {}{}", keys.0, keys.1);

    // let we_load: WebEvent = WebEvent::WELoad(true);
    // let we_click: WebEvent = WebEvent::WEClick(click);
    // let we_key: WebEvent = WebEvent::WEKeys(keys);

    // println!(
    //     "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
    //     we_load, we_click, we_key
    // );

    // let casual: &str = "Casual: See you later!";
    // goodbye("Formal: Good bye.");
    // goodbye(casual);

    // let num = 25;
    // println!("{} divided by 5 = {}", num, divide_by_5(num));
    println!("is: {} ", is_true());
}

// fn goodbye(message: &str) {
//     println!("\n{}", message);
// }

// fn divide_by_5(num: u32) -> u32 {
//     if num == 0 {
//         // Return early
//         return 0;
//     }
//     num / 5
// }

fn is_true() -> bool {
    return true;
}
