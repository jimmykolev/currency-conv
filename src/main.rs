use std::io;


fn main() {
    println!("#######################");
    println!("Currency Converter GBP");
    println!("#######################");
    let mut currency = String::new();
    let mut amount = String::new();
    println!("Please enter the currency you want to convert to: ");
    println!("1. USD     2. EUR\n3. LEV     4. INR\n5. JPY     6. CHF\n7. CAD     8. AUD\n9. CNY     10. RUB");
    io::stdin().read_line(&mut currency).unwrap();
    println!("Please enter the amount you want to convert: ");
    io::stdin().read_line(&mut amount).unwrap();
    let amount_int:f64 = amount.trim().parse().unwrap();
    match currency.trim().parse().unwrap() {
        1 => println!("{} GBP is {} USD", amount_int, convert(amount_int, &currency)),
        2 => println!("{} GBP is {} EUR", amount_int, convert(amount_int, &currency)),
        3 => println!("{} GBP is {} LEV", amount_int, convert(amount_int, &currency)),
        4 => println!("{} GBP is {} INR", amount_int, convert(amount_int, &currency)),
        5 => println!("{} GBP is {} JPY", amount_int, convert(amount_int, &currency)),
        6 => println!("{} GBP is {} CHF", amount_int, convert(amount_int, &currency)),
        7 => println!("{} GBP is {} CAD", amount_int, convert(amount_int, &currency)),
        8 => println!("{} GBP is {} AUD", amount_int, convert(amount_int, &currency)),
        9 => println!("{} GBP is {} CNY", amount_int, convert(amount_int, &currency)),
        10 => println!("{} GBP is {} RUB", amount_int, convert(amount_int, &currency)),
        _ => println!("Invalid currency"),
    }
        
}

fn convert(amount: f64, currency:&String) -> f64 {
    match currency.trim().parse().unwrap() {
        1 => amount * 1.23,
        2 => amount * 1.15,
        3 => amount * 2.24,
        4 => amount * 101.34,
        5 => amount * 161.49,
        6 => amount * 1.14,
        7 => amount * 1.68,
        8 => amount * 1.83,
        9 => amount * 8.45,
        10 => amount * 94.86,
        _ => 0.0,
    }
    
}