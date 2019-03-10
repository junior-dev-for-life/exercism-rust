pub fn find() -> Option<u32> {
    let sum = 1000;

    for a in 1..sum/2 {
        for b in 1..sum/3 {
            let c = sum - a - b;
            if (a*a + b*b) == c*c {
                return Some(a * b * c);
            }
        }
    }

    return None;
}
