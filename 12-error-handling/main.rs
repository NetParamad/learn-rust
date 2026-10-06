// 12 - error handling โน้ต: Option/Result + match, ? ส่ง error ขึ้น caller
use std::num::ParseIntError;

// ? ในฟังก์ชันที่คืน Result: พังตรงไหน return Err ทันที (แทน throw)
fn double_parsed(s: &str) -> Result<u32, ParseIntError> {
    let n = s.parse::<u32>()?;
    Ok(n * 2)
}

fn main() {
    // Result จาก parse: match แยก Ok/Err (แทน try/catch)
    match "25".parse::<u32>() {
        Ok(n) => println!("parsed: {n}"),
        Err(e) => println!("parse failed: {e}"),
    }

    // Option แทน null: unwrap_or คือ ?? ของ JS
    let maybe: Option<&str> = None;
    println!("maybe: {}", maybe.unwrap_or("default"));

    // ? กระจาย error: "21" ผ่าน, "abc" ได้ Err โดยไม่ panic
    match double_parsed("21") {
        Ok(n) => println!("21 x 2 = {n}"),
        Err(e) => println!("failed: {e}"),
    }
    match double_parsed("abc") {
        Ok(n) => println!("abc x 2 = {n}"),
        Err(e) => println!("failed: {e}"),
    }
}
