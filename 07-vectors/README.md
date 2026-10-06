# 07 Vectors

## ภาษาไทย
โน้ต: `Vec` คือ array ที่โตได้, `vec![]` สร้างพร้อมค่า, index เกินแล้ว panic (ใช้ `.get()` ได้ `Option` แทน), copy ด้วย `.clone()`

รัน: `rustc --edition 2021 main.rs -o main && ./main`

## English
Note: `Vec` is a growable array, `vec![]` creates with values, out-of-bounds indexing panics (use `.get()` for `Option` instead), copy with `.clone()`

Run: `rustc --edition 2021 main.rs -o main && ./main`
