# 08 Structs

## ภาษาไทย
โน้ต: `struct` แทน object, `impl` ใส่ method (`&self` คือ `this`), `..base` ก็อป field ที่เหลือ (ต้อง `clone` เพราะ `String` ไม่ใช่ `Copy`)

รัน: `rustc --edition 2021 main.rs -o main && ./main`

## English
Note: `struct` replaces object, `impl` adds methods (`&self` is `this`), `..base` copies remaining fields (needs `clone` since `String` is not `Copy`)

Run: `rustc --edition 2021 main.rs -o main && ./main`
