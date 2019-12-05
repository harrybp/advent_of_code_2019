fn main() {
    let lower_bound: i32 = 206938;
    let upper_bound: i32 = 679128;

    let mut count: i32 = 0;

    for i in lower_bound..upper_bound {
        if check_number(i) {
            count += 1;
        }
    }
    println!("There are {} possible passwords", count);
}

fn check_number(mut rest: i32) -> bool {
    let mut number;
    let mut previous_digit = 0;
    let mut previous_previous_digit = 0;
    let mut repeat = false;
    let mut repeat_digit = 0;
    for i in [100000, 10000, 1000, 100, 10, 1].iter() {
        //Calculate digit
        number = rest;
        rest = rest % i;
        let digit = (number - rest) / i;

        //Fail if lower than previous digit
        if digit < previous_digit {
            return false;
        }

        //Record a single-repeating digit
        if (digit == previous_digit) && (digit != previous_previous_digit) & !repeat {
            repeat = true;
            repeat_digit = digit;
        }

        //Remove a double-repeating digit
        if (digit == previous_digit) && (digit == previous_previous_digit) && (digit == repeat_digit) {
            repeat = false;
        }

        previous_previous_digit = previous_digit;
        previous_digit = digit;
    }

    return repeat;
}
