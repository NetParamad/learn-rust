// 10 - ownership โน้ต: move vs Copy vs clone, ส่งเข้าฟังก์ชันก็ move ได้
fn takes_ownership(s: String) {
    println!("took: {s}");
} // s หมดอายุตรงนี้ memory คืนอัตโนมัติ (ไม่ต้อง free/gc)

fn makes_copy(n: i32) {
    println!("copied: {n}");
}

fn main() {
    // String: assign = move เจ้าของย้าย ตัวเก่าตาย
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{s1}"); // Error: borrow of moved value
    println!("s2: {s2}");

    // i32 เป็น Copy: assign = ก็อปค่า ไม่ move
    let n1 = 5;
    let n2 = n1;
    println!("{n1} {n2} (ok ทั้งคู่)");

    // clone = deep copy ชัดเจน ทั้งคู่ยังใช้ได้
    let s3 = s2.clone();
    println!("s2: {s2}, s3: {s3}");

    takes_ownership(s3); // move เข้าฟังก์ชัน
    // println!("{s3}"); // Error: s3 ถูก move ไปแล้ว

    makes_copy(n1); // Copy ไม่ move
    println!("n1 ยังอยู่: {n1}");
}
