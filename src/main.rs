fn main() {
    let mut num: i8 = 0;

    let mut is_three: bool;
    let mut is_five: bool;

    loop {
        (is_three, is_five) = (false, false);

        if num % 3 == 0 {
            is_three = true;
        }

        if num % 5 == 0 {
            is_five = true;
        }

        if is_three && is_five {
            println!("Fizz Buzz");
        } else if is_three {
            println!("Fizz");
        } else if is_five {
            println!("Buzz");
        } else {
            println!("{}", num);
        }

        num += 1;

        if num == 101 {
            return;
        }
    }
}
