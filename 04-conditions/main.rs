// 04 - conditions โน้ต: if คืนค่าได้, match ต้องครบทุกเคส
fn main() {
    let n = 7;

    if n % 2 == 0 {
        println!("{n} is even");
    } else {
        println!("{n} is odd");
    }

    // if เป็น expression: assign ผลลัพธ์ได้เลย (แทน ternary ? : ของ JS)
    let label = if n > 10 { "big" } else { "small" };
    println!("{n} is {label}");

    // match แทน switch: | คือ or-pattern, _ ดักที่เหลือ (ห้ามลืม ไม่งั้น compile ไม่ผ่าน)
    let day = 3;
    let name = match day {
        1 => "Mon",
        2 | 3 => "Tue/Wed",
        4..=7 => "rest of week", // range pattern
        _ => "invalid",
    };
    println!("day {day} = {name}");
}
