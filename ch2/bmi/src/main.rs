fn input(prompt: &str) -> f64 {
    // print prompt
    println!("{}", prompt);
    // get input value
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("input Error");
    // remove whitespace and change string to float
    return s.trim().parse().expect("value is not number");
}

fn main() {
    // input height and weight
    let height_cm = input("height(cm): ");
    let weight_kg = input("weight(cm): ");
    // calculate bmi
    let bmi = weight_kg / (height_cm / 100.0).powf(2.0);
    println!("BMI={:.1}", bmi);
    // estimate bmi
    if bmi < 18.5 {
        println!("low");
    } else if bmi < 23.0 {
        println!("common");
    } else if bmi < 25.0 {
        println!("warning");
    } else if bmi < 30.0 {
        println!("1st level");
    } else if bmi < 35.0 {
        println!("2nd level");
    } else {
        println!("Fucked up");
    }
}
