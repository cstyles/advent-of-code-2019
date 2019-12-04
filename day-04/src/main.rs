

fn main() {
    let range = 357253..892942+1;

    let mut valid_passwords: Vec<i32> = vec![];

    for i in range {
        if valid(digits(i)) {
            valid_passwords.push(i);
        }
    }

    println!("valid_passwords.len(): {}", valid_passwords.len());
    // println!("valid_passwords: {:?}", valid_passwords);
}

fn valid(digits: Vec<i32>) -> bool {
    adjacent_check(&digits) && non_decrease_check(&digits)
}

fn adjacent_check(digits: &Vec<i32>) -> bool {
    for i in 0..5 {
        if digits[i] == digits[i+1] {
            return true
        }
    }

    false
}

fn non_decrease_check(digits: &Vec<i32>) -> bool {
    for i in 0..5 {
        if digits[i] > digits[i+1] {
            return false
        }
    }

    true
}

fn digits(num: i32) -> Vec<i32> {
    let mut ugh = num;

    let ones = ugh % 10;
    ugh /= 10;

    let tens = ugh % 10;
    ugh /= 10;

    let hundreds = ugh % 10;
    ugh /= 10;

    let thousands = ugh % 10;
    ugh /= 10;

    let ten_thousands = ugh % 10;
    ugh /= 10;

    let hundred_thousands = ugh % 10;

    vec![
        hundred_thousands,
        ten_thousands,
        thousands,
        hundreds,
        tens,
        ones,
    ]
}

