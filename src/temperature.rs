use std:: io;

pub fn convert_celsius_to_fahrenheit() {
    println!("摂氏(°C)を入力してください:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("読み込みに失敗しました。");

    let input: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("無効な数値です。");
            return;
        }
    };

    let fahrenheit = celsius_to_fahrenheit(input);
    println!("{}°C は {}°F です。", input, fahrenheit);
}

pub fn convert_fahrenheit_to_celsius() {
    println!("華氏(°F)を入力してください:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("読み込みに失敗しました。");

    let input: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("無効な数値です。");
            return;
        }
    };

    let celsius = fahrenheit_to_celsius(input);
    println!("{}°F は {}°C です。", input, celsius);
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
