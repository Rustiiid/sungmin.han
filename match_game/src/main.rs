use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("숫자를 맞혀봅시다!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("정답이라고 생각하는숫자를 입력하세요.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("입력한값을읽지 못했습니다.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("입력한 값: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한숫자가작습니다!"),
            Ordering::Greater => println!("입력한숫자가큽니다! "),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    }
}
