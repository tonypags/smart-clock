use chrono::Local;

fn main() {
    let name = "Tony";

    println!("Hi {0}!", name);
    println!("This time is {}", Local::now());
}
