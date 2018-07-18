// mod random_number;
// mod hello_world;
// mod guess;
// mod expression;
// mod owner_ship;

mod japanese;
mod english;

fn main() {
    println!("Hello in English: {}", english::greetings::hello());
    println!("Goodbye in English: {}", english::farewells::goodbye());

    println!("Hello in Japanese: {}", japanese::greetings::hello());
    println!("Goodbye in Japanese: {}", japanese::farewells::goodbye());

    // owner_ship::owner_ship();
    // expression::expression();
    // hello_world::hello_world();
    // guess::guess();
}
