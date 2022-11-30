mod car_factory {
    pub fn build_car() {
        //make this pub to can access in the main function
        println!("Honk honk!");
    }
}

fn main() {
    car_factory::build_car();
}
