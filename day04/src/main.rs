fn is_valid(mut num: u64) -> (bool, bool) {
    let mut last_digit = 10;
    let mut cond1 = false;
    let mut cond2 = true;
    let mut cond3 = false;

    let mut span = 0;

    while num > 0 {
        let digit = num % 10;

        if digit == last_digit {
            span += 1;
            if span == 2 {
                cond1 = true;
            }
        } else {
            if span == 2 {
                cond3 = true;
            }
            span = 1;
        }

        if digit > last_digit {
            cond2 = false;
            break;
        }

        last_digit = digit;
        num = num / 10;
    }

    if span == 2 {
        cond3 = true;
    }

    ((cond1 && cond2), (cond3 && cond2))
}

fn main() {
    let range = 108457..562041;
    let mut num_valid1 = 0;
    let mut num_valid2 = 0;
    for num in range {
        let (valid1, valid2) = is_valid(num);
        if valid1 {
            num_valid1 += 1;
        }
        if valid2 {
            num_valid2 += 1;
        }
    }
    println!("Valid 1: {}", num_valid1);
    println!("Valid 2: {}", num_valid2);
}
