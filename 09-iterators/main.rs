// 09 - iterators โน้ต: map/filter/find/fold, ขี้เกียจ ต้อง collect มาจบ
fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    // map แปลงทุกตัว
    let doubled: Vec<i32> = nums.iter().map(|x| *x * 2).collect();
    println!("map: {doubled:?}");

    // filter กรอง (|&&x| แกะ & ซ้อนกันให้เหลือ i32)
    let evens: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("filter: {evens:?}");

    // find เอาตัวแรกที่เข้าเงื่อนไข (ได้ Option เพราะอาจไม่เจอ)
    let first_big = nums.iter().find(|&&x| x > 3);
    println!("find >3: {first_big:?}");

    // reduce รวมยอด: fold ระบุค่าเริ่ม หรือ sum ตรงๆ
    let total: i32 = nums.iter().sum();
    let folded = nums.iter().fold(0, |acc, x| acc + x);
    println!("sum: {total}, fold: {folded}");
}
