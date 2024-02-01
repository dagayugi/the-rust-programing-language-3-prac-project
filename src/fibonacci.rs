use std:: io;

pub fn generate_the_nth_fibonacci() {
    println!("フィボナッチ数列のn番目の数を知りたいです。nを入力してください");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("読み込みに失敗しました。");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("無効な数値です。");
            return;
        }
    };

    let result = fibonacci(input);
    println!("フィボナッチ数列の{}番目の数: {}", input, result);
}

fn fibonacci(n: u32) -> u32{
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}
