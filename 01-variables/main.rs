// 01 - variables โน้ต: mut ถึงแก้ได้, const ห้าม assign ใหม่, shadowing ประกาศทับชื่อเดิมได้
fn main() {
    let name = "Net";
    // name = "Paramad"; // Error: cannot assign twice to immutable variable

    let mut city = "Bangkok";
    let before = city; // อ่านค่าแรกเก็บไว้ (&str เป็น Copy)
    city = "Chiang Mai"; // ok - mut แก้ค่าได้ (แต่เปลี่ยน type ไม่ได้)

    const AGE: u32 = 25;
    // AGE = 26; // Error: cannot assign to constant

    let score = 10;
    let score = score + 5; // shadowing: ประกาศใหม่ทับชื่อเดิม
    let score = score.to_string(); // shadowing เปลี่ยน type ได้ด้วย (mut ทำแบบนี้ไม่ได้)

    println!("name: {name}");
    println!("city: {before} -> {city}");
    println!("AGE: {AGE}");
    println!("score: {score}");
}
