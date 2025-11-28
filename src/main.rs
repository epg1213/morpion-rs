mod morpion;
use morpion::{Morpion, Player};
use morpion::get_input;

fn main() {
    let mut morpion = Morpion::new();
    println!("Autoplay ? (y/n)");
    loop {
        match get_input().as_str() {
            "y" => {
                morpion.set_autoplay();
                break;
            },
            "n" => break,
            _ => println!("Please type y or n.")
        }
    }
    let mut winner = None;
    println!("{}", morpion);
    while winner == None {
        winner = morpion.next();
        println!("{}", morpion);
    }
    match winner {
        Some(Player::Cross) => println!("Cross wins !"),
        Some(Player::Circle) => println!("Circle wins !"),
        _ => println!("Draw !"),
    }
}


