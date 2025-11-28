mod morpion;
use morpion::Morpion;

fn main() {
    let mut morpion = Morpion::new(true);
    let mut winner = None;
    println!("{}", morpion);
    while winner == None {
        winner = morpion.next();
        println!("{}", morpion);
    }
    match winner {
        Some(' ') => println!("Draw !"),
        Some('X') => println!("Cross wins !"),
        Some('O') => println!("Circle wins !"),
        _ => println!("Draw !"),
    }
}


