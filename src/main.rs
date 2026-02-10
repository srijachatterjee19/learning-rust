fn average_price() -> f64 {
    let fruit_name = "banana";
    let quantity = 5;
    let eu_price = 3.5;
    let us_price = 2.0;
    let price = if quantity > 20 { eu_price } else { us_price };
    price // Return the price
}

fn average(numbers: &[f64]) -> f64 {
    let mut sum = 0.0;
    for num in numbers {
        sum += num;
    }

    for num in numbers {
        println!("{}", num);
    }
    sum / numbers.len() as f64
}

fn main() {
    println!("Hello, world!");

    let item = "mango";
    let price = 2.5;
    let quantity = 10;

    println!("{} {} for {} dollars", quantity, item, price);
}
