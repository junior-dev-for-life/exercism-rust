pub fn square_of_sum(n: i32) -> i32 {
    let sum = (1..=n).fold(0, |total, next| total + next);
    return sum * sum;
}

pub fn sum_of_squares(n: i32) -> i32 {
    let sum = (1..=n).fold(0, |total, next| total + (next * next));
    return sum;
}

pub fn difference(n: i32) -> i32 {
    return square_of_sum(n) - sum_of_squares(n);
}
