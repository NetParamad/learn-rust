# 10 Ownership

## ภาษาไทย
โน้ต: หัวใจของ Rust — `String` ย้ายเจ้าของ (`move`) แล้วตัวเก่าใช้ต่อไม่ได้, type เล็ก (`i32`) เป็น `Copy` เลยไม่ move, `.clone()` ก็อปปี้ลึกแบบชัดเจน

รัน: `rustc --edition 2021 main.rs -o main && ./main`

## English
Note: the heart of Rust — `String` assignment moves ownership so the old variable is dead, small types (`i32`) are `Copy` and don't move, `.clone()` makes an explicit deep copy

Run: `rustc --edition 2021 main.rs -o main && ./main`
