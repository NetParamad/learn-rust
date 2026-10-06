// 02 - data types โน้ต: scalar + tuple + array, String (owned) vs &str (borrowed)
fn main() {
    // scalar: จำนวนเต็ม / ทศนิยม / bool / ตัวอักษร
    let a: i32 = -42;
    let b: u64 = 42;
    let c: f64 = 3.14;
    let d: bool = true;
    let e: char = 'ก'; // char 4 bytes เก็บ unicode ได้ (ต่างจาก JS string[0])

    // tuple: รวมหลาย type ในก้อนเดียว
    let tup: (i32, f64, char) = (500, 6.4, 'z');
    let (x, y, z) = tup; // destructuring
    println!("tuple: {x} {y} {z}, เข้าถึงตรงๆ: {}", tup.0);

    // array: type เดียวกัน + ความยาวตายตัว (อยู่บน stack)
    let arr: [i32; 3] = [1, 2, 3];
    println!("array[0]: {}, len: {}", arr[0], arr.len());

    // String vs &str: String เป็นเจ้าของข้อมูล ขยายได้ / &str แค่ยืมมอง
    let slice: &str = "hello";
    let mut owned: String = String::from("hello");
    owned.push_str(" rust"); // ขยายได้เฉพาะ String
    println!("&str: {slice}, String: {owned}");

    println!("scalar: {a} {b} {c} {d} {e}");
}
