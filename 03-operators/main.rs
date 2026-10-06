// 03 - operators โน้ต: หาร int ตัดเศษ, ไม่มี ++/--, == เช็คค่าตรงๆ ได้เลย
fn main() {
    let a = 10;
    let b = 3;

    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {} (ตัดเศษ!)", a, b, a / b);
    println!("{} % {} = {}", a, b, a % b);

    // เปรียบเทียบได้ bool
    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);
    println!("a > b: {}", a > b);

    // ตรรกะ && || !
    println!("true && false: {}", true && false);
    println!("true || false: {}", true || false);
    println!("!true: {}", !true);

    // compound assign (ไม่มี ++/-- ใน Rust)
    let mut n = 5;
    n += 1; // แทน n++
    n *= 2;
    println!("n: {n}");
}
