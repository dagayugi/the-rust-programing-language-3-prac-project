mod temperature;
mod fibonacci;

use std:: io;

fn main() {
    // 温度・フィボナッチ数列・クリスマスキャロル
    loop {
        println!("変換タイプを選択してください:");
        println!("1: 摂氏から華氏へ");
        println!("2: 華氏から摂氏へ");
        println!("3: フィボナッチ数列のn番目を生成する");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("読み込みに失敗しました。");


        match choice.trim() {
            "1" => temperature::convert_celsius_to_fahrenheit(),
            "2" => temperature::convert_fahrenheit_to_celsius(),
            "3" => fibonacci::generate_the_nth_fibonacci(),
            _ => continue,
        }
        break;
    }
}
