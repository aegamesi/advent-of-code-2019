fn is_valid(mut num: u64) -> bool {
    let mut last_digit = 10;
    let mut cond1 = false;
    let mut cond2 = true;

    while num > 0 {
        let digit = num % 10;

        if digit == last_digit {
            cond1 = true;
        }
        if digit > last_digit {
            cond2 = false;
            break;
        }

        last_digit = digit;
        num = num / 10;
    }

    cond1 && cond2
}

fn main() {
    let range = 108457..562041;
    let mut valid = 0;
    for num in range {
        if is_valid(num) {
            valid += 1;
        }
    }
    println!("Valid: {}", valid);
}
