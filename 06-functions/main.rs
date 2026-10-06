// 06 - functions โน้ต: fn ต้องระบุ type, ไม่มี ; ท้าย = return, closure แทน arrow
fn add(a: i32, b: i32) -> i32 {
    a + b // ไม่มี ; แปลว่า return ค่านี้
}

fn greet(name: &str) -> String {
    format!("hi {name}")
}

fn main() {
    println!("{}", add(2, 3));

    // closure แทน arrow fn ของ JS: |param| body
    let multiply = |a: i32, b: i32| a * b;
    println!("{}", multiply(2, 3));

    // Rust ไม่มี default param: ใช้ Option + unwrap_or กัน None
    let greet_or_guest = |name: Option<&str>| format!("hi {}", name.unwrap_or("guest"));
    println!("{}", greet_or_guest(None));
    println!("{}", greet_or_guest(Some("Net")));

    println!("{}", greet("Paramad"));
}
