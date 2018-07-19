// mod random_number;
// mod hello_world;
// mod guess;
// mod expression;
// mod owner_ship;

mod phrases;

fn main() {
    println!("Hello in English: {}", phrases::english::greetings::hello());
    println!("Goodbye in English: {}", phrases::english::farewells::goodbye());

    println!("Hello in Japanese: {}", phrases::japanese::greetings::hello());
    println!("Goodbye in Japanese: {}", phrases::japanese::farewells::goodbye());

    // owner_ship::owner_ship();
    // expression::expression();
    // hello_world::hello_world();
    // guess::guess();
}
