fn main() {
    let formal: bool = true;
    let gretting = if formal {
        "Good day to you."
    } else {
        "What's up?"
    };

    println!("{}", gretting);

    let num: i32 = 500;
    let out_of_range: bool;

    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }

    println!("{} is out of range? {}", num, out_of_range);
}
