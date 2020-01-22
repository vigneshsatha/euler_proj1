fn main() {
    let n = 4000000;
    let mut sum = 0;
    let mut first = 1;
    let mut second = 2;
    while first <= n {
        if first % 2 == 0 {
            sum += first;
        }
        let temp = first;
        first = second;
        second += temp;        
    }
    println!("Sum = {}", sum);
}
