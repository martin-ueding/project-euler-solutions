pub fn solution() -> i32 {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 1;

    loop {
        if a > 4_000_000 {
            break;
        }
        if a % 2 == 0 {
            sum += a;
        }
        let c = a + b;
        a = b;
        b = c;
    }

    sum
}
