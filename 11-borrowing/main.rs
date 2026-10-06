// 11 - borrowing โน้ต: & ยืมอ่าน / &mut ยืมเขียน (ตัวเดียว, ต้อง mut ทั้งคู่)
fn calc_len(s: &str) -> usize {
    s.len() // ยืมอ่าน ไม่กิน ownership (รับ &str จะยืดหยุ่นกว่า &String)
}

fn add_suffix(s: &mut String) {
    s.push_str(" rust"); // ยืมเขียน: ต้อง mut ทั้ง param และคนเรียก
}

fn main() {
    let mut s = String::from("hi");

    let len = calc_len(&s); // ยืมแบบอ่าน
    println!("{s} len={len} (s ยังใช้ได้)");

    add_suffix(&mut s); // ยืมแบบเขียน
    println!("after: {s}");

    // กฎ borrow: & หลายตัวพร้อมกันได้
    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}"); // ใช้ r1/r2 ครั้งสุดท้ายตรงนี้ -> borrow หมดอายุ

    // & เก่าหมดอายุแล้ว ถึงยืม &mut ใหม่ได้ (ห้ามมี &mut ค้างพร้อมตัวอื่น)
    let r3 = &mut s;
    r3.push('!');
    println!("{r3}");
}
