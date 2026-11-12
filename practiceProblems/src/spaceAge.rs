use std::io;

fn main() {
    println!("This programs calculates the age of the a person on a particular planet.");
    println!("Just enter the age of the person and the planet name you want to go (Make sure the spelling is correct).");

    let mut age = String::new();
    println!("Please enter your age (in years) on earth:");
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line.");
    let age: i32 = age.trim().parse().expect("Try a numerical value bro!");

    println!("Please enter your planet name: ");
    let mut planet = String::new();
    io::stdin()
        .read_line(&mut planet)
        .expect("Failed to read line.");
    let planet = planet.trim().to_lowercase();

    let planets = [
        "mercury", "venus", "earth", "mars", "jupiter", "saturn", "uranus", "neptune",
    ];
    if !planets.contains(&planet.as_str()) {
        println!(
            "Dont make a mess bro, just enter the name of the planet that is in out solar system."
        );
    }

    println!(
        "The selected age is {} and the selected planet is {}.",
        age, planet
    );

    println!(
        "Your age on planet {} will be {}",
        planet,
        calc_age(&planet, age)
    );
}

fn calc_age(planet: &str, age: i32) -> f64 {
    let mut age = age as f64;
    age = match planet {
        "mercury" => age / 0.2408467,
        "venus" => age / 0.61519726,
        "earth" => age,
        "mars" => age / 1.8808158,
        "jupiter" => age / 11.862615,
        "saturn" => age / 29.447498,
        "uranus" => age / 84.016846,
        "neptune" => age / 164.79132,
        _ => -1.0,
    };
    age
}
