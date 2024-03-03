fn main() {
    println!("Hello, world!");
    let x: i32 = 10;
    println!("x: {x}");
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("final sentence: [{}]", sentence);
    println!("length sentence: {}", sentence.len());
    println!("other `{:?}", sentence.bytes());
    // println!("{:?}", &sentence[12..13]);
}
