# 15 Product Search CLI

## ภาษาไทย
ค้นหา + กรองหมวด (in-memory): พิมพ์คำค้นกรองชื่อ, ใส่หมวดตัวที่สองกรอง category

โน้ต: `filter` + `contains`, ตัดหมวดซ้ำด้วย `contains` check, `to_lowercase` เทียบแบบไม่สนตัวพิมพ์

รัน:
- `rustc --edition 2021 main.rs -o main && ./main` (ดูทั้งหมด)
- `./main laptop` / `./main "" electronics`

## English
Search + category filter (in-memory): query filters by name, optional second arg filters category.

Note: `filter` + `contains`, dedupe categories with a `contains` check, `to_lowercase` for case-insensitive match

Run:
- `rustc --edition 2021 main.rs -o main && ./main` (see all)
- `./main laptop` / `./main "" electronics`
