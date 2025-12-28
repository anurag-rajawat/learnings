use std::thread;

#[derive(Debug)]
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    // println! is known as a macro. A macro is a function that writes code for you; all macros have a ! after them.
    // You don’t need to worry about remembering to add the ! because the compiler will notice if you don’t.
    println!("Namaste, world!");

    // Dangling pointer example
    // let mut grains: Vec<Cereal> = vec![];
    // grains.push(Cereal::Barley);
    // drop(grains);
    // println!("{:?}", grains);

    // Race condition example
    // let mut data = 100;
    // thread::spawn(|| {data = 500;});
    // thread::spawn(|| { data = 1000;});
    // println!("{}", data);

    // Buffer overflow example
    // let fruits = vec!["Apple", "Banana", "Grapes", "Orange"];
    // let buffer_overflow = fruits[10];
    // assert_eq!(buffer_overflow, "Orange");

    // Attempting to modify an iterator while iterating over it
    // let mut letters = vec!["A", "B", "C", "D", "E"];
    // for letter in letters {
    //     println!("Letters: {}", letter);
    //     letters.push(letter.clone())
    // }
}
