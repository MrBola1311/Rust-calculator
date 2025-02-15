use std::io;

// Temel matematik işlemlerini temsil eden enum
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Hesaplama işlemini gerçekleştiren fonksiyon
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => x / y,
    }
}

fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");
    input.trim().parse().expect("Geçerli bir sayı giriniz")
}

fn main() {
    let num1 = get_input("İlk sayıyı girin:");
    
    println!("İşlemi girin (+, -, *, /):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Okuma hatası");
    
    let num2 = get_input("İkinci sayıyı girin:");

    let operation = match operator.trim() {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => panic!("Geçersiz işlem!"),
    };

    println!("Sonuç: {}", calculate(operation));
}
