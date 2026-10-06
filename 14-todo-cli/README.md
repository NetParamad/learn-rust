# 14 Todo CLI

## ภาษาไทย
Todo + ไฟล์ `todos.txt` (แทน localStorage): เพิ่ม / ลิสต์ / ติ๊กเสร็จ / ลบ งานไม่หายเมื่อรันใหม่

โน้ต: `env::args`, `fs::read/write`, `split_once`, `filter_map`

รัน:
- `rustc --edition 2021 main.rs -o main && ./main add "อ่าน Rust"`
- `./main list` / `./main done 1` / `./main del 1`

## English
Todo + `todos.txt` file (instead of localStorage): add / list / check / delete, persists between runs.

Note: `env::args`, `fs::read/write`, `split_once`, `filter_map`

Run:
- `rustc --edition 2021 main.rs -o main && ./main add "read Rust"`
- `./main list` / `./main done 1` / `./main del 1`
