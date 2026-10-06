# 11 Borrowing

## ภาษาไทย
โน้ต: `&` ยืมอ่านกี่ตัวก็ได้, `&mut` ยืมเขียนได้ตัวเดียวและห้ามมีตัวอื่นค้าง, ใช้เสร็จแล้ว borrow หมดอายุ (ยืมใหม่ได้)

รัน: `rustc --edition 2021 main.rs -o main && ./main`

## English
Note: many `&` readers allowed, only one `&mut` writer and nothing else alive beside it, borrows expire when last used (then you can borrow again)

Run: `rustc --edition 2021 main.rs -o main && ./main`
