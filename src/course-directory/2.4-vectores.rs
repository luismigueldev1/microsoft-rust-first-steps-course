fn main() {
    let three_nums: Vec<i32> = vec![15, 3, 46];
    let zeroes: Vec<i32> = vec![0; 3];
    println!("{:?}", zeroes);
    println!("{:?}", three_nums);

    let mut fruits: Vec<&str> = Vec::new();
    fruits.push("Apple");
    fruits.push("Banana");
    fruits.push("Cherry");
    println!("{:?}", fruits);
    fruits.pop();
    println!("{:?}", fruits);

    let mut index_vec: Vec<i32> = vec![15, 3, 46];
    let three: i32 = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);

    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);
}
