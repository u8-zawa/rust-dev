use std::io::{self, Write};

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    println!("素数判定プログラム");
    print!("数値を入力してください: ");
    io::stdout().flush().unwrap();

    // let mut input = String::new();
    let mut input = "10089886811898868001".to_string();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<u64>() {
        Ok(number) => {
            if is_prime(number) {
                println!("{} は素数です！", number);
            } else {
                println!("{} は素数ではありません。", number);
            }
        }
        Err(_) => println!("有効な数値を入力してください。"),
    }
}