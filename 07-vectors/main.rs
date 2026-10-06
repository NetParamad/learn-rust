// 07 - vectors โน้ต: Vec = array โตได้, push/len, .get() กัน index เกิน
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);

    let v2 = vec![1, 2, 3]; // macro สร้างพร้อมค่า
    println!("len: {}", v2.len());
    println!("first: {}, last: {}", v2[0], v2[v2.len() - 1]);

    // v2[10]; // panic! index เกิน — ใช้ .get() ได้ Option มากันพังแทน
    match v2.get(10) {
        Some(x) => println!("found {x}"),
        None => println!("index 10: none (ไม่ panic)"),
    }

    let copy = v2.clone(); // copy ทั้งก้อนด้วย .clone() (แทน [...arr])
    println!("v2: {v2:?}, copy: {copy:?}");
}
