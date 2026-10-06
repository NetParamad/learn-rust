# 13 Mini Project: Text Stats

## ภาษาไทย
เป้าหมาย portfolio: รับชื่อไฟล์ -> นับบรรทัด/คำ/ตัวอักษร (ไม่ใส่ arg จะนับข้อความตัวอย่างแทน)

โน้ตรวมยอด: `env::args` + `fs::read_to_string` + `lines/count` + `split_whitespace` + `chars`

รัน: `rustc --edition 2021 main.rs -o main && ./main main.rs`

## English
Portfolio goal: take a filename -> count lines/words/chars (no arg counts the sample text instead).

Key points: `env::args` + `fs::read_to_string` + `lines/count` + `split_whitespace` + `chars`

Run: `rustc --edition 2021 main.rs -o main && ./main main.rs`
