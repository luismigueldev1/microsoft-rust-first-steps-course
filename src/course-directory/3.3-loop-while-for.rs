fn main() {
    // loop {
    //     println!("We loop forever!");
    //     break;
    // }
    // let mut counter = 1;
    // let stop_loop = loop {
    //     counter *= 2;
    //     if counter > 100 {
    //         break counter;
    //     }
    // };
    // println!("Break the loop at counter = {}.", stop_loop);

    //while
    // let mut counter: i32 = 1;
    // while counter < 5 {
    //     println!("We loop a while...");
    //     counter = counter + 1;
    // }

    //for
    //let big_birds: [&str; 3] = ["ostrich", "peacock", "stork"];
    // for bird in big_birds.iter() {
    //     println!("The {} is a big bird", bird);
    // }
    for number in 0..5 {
        println!("{}", number * 2);
    }
}
