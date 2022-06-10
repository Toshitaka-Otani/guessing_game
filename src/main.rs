use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("数を当ててみよう！");

    let secret_number = rand::thread_rng().gen_range(1..=100);   //memo：上限値を含まないため普通に書くと101と書かないと100までを乱数に含めることができない
    
    loop {
        println!("予想を入力しよう！");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました。");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("あなたの予想： {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さい!"),
            Ordering::Greater => println!("大きい!"),
            Ordering::Equal => {
                println!("当たり！");
                break;
            }
        };
    }
}