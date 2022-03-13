use std::io::{self, Write};

fn main() {
    print!("Введите число для проверки его простоты: ");
    match io::stdout().flush() {
        Ok(_) => (),
        Err(_) => {}
    }

    let mut buf = String::new();
    
    if io::stdin().read_line(&mut buf).is_ok() {
        let parsed_n = buf.trim().parse::<i32>();

        if parsed_n.is_ok() {
            let n: i32 = buf.trim().parse().unwrap();
            let mut denominator: i32 = -1;
            if !is_simple(n, &mut denominator) {
                if n <= 1 {
                    println!("Введенное число не натуральное или в пределах [0; 1]")
                } else {
                    println!("У числа есть делитель: {}", denominator);
                }
            } else {
                println!("Число простое")
            }
        } else {
            println!("Введено некорректное число");
        }
    } else {
        println!("Введено некорректное число");
    }
    
    println!("end:-)")
}

// Возвращает true если натуральное число простое
fn is_simple(n: i32, denominator: &mut i32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            *denominator = i;
            return false;
        }
    }
    n > 1
}
