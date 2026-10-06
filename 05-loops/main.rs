// 05 - loops โน้ต: loop/while/for, break พร้อมค่า, range .. กับ ..=
fn main() {
    // loop: วนไม่รู้จบจนกว่าจะ break (พร้อมคืนค่าได้)
    let mut i = 0;
    loop {
        i += 1;
        if i == 3 {
            continue; // ข้ามรอบนี้
        }
        if i > 5 {
            break;
        }
        println!("loop i={i}");
    }

    let answer = loop {
        break 42; // loop เป็น expression คืนค่าได้
    };
    println!("answer: {answer}");

    // while
    let mut n = 3;
    while n > 0 {
        println!("while n={n}");
        n -= 1;
    }

    // for + range: 0..5 = 0-4, 0..=5 = 0-5
    for x in 0..3 {
        println!("range x={x}");
    }

    // enumerate ได้ (index, ค่า) พร้อมกัน
    for (idx, fruit) in ["apple", "mango"].iter().enumerate() {
        println!("{idx}: {fruit}");
    }
}
