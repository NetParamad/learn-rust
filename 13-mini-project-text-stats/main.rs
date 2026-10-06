// 13 - mini project: ตัวนับบรรทัด/คำ/ตัวอักษร (CLI)
// รัน: ./main main.rs (นับไฟล์ตัวเอง) หรือ ./main (นับข้อความตัวอย่าง)
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // มี arg = อ่านไฟล์, ไม่มี = ใช้ข้อความตัวอย่าง
    let (label, text) = if args.len() > 1 {
        let path = &args[1];
        match fs::read_to_string(path) {
            Ok(t) => (path.clone(), t),
            Err(e) => {
                eprintln!("open '{path}' failed: {e}");
                std::process::exit(1);
            }
        }
    } else {
        (
            String::from("<sample>"),
            String::from("hello rust\nlearn ownership and borrowing\n"),
        )
    };

    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();

    println!("file: {label}");
    println!("lines: {lines}");
    println!("words: {words}");
    println!("chars: {chars}");
}
