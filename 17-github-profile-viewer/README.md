# 17 GitHub Profile Viewer

## ภาษาไทย
พิมพ์ username -> ดู followers / จำนวน repo + 5 repo ล่าสุดพร้อมดาว

โน้ต: `fetch` 2 ต่อ, เช็ค 404 ก่อน (`error_for_status`), GitHub บังคับส่ง `User-Agent`, `Option` รับ field ที่ขาดได้

รัน: `cargo run -- torvalds` (ครั้งแรกโหลด dependency ต้องมีเน็ต)

## English
Type a username -> see followers / repo count + 5 latest repos with stars.

Note: chained `fetch`, check 404 first (`error_for_status`), GitHub requires a `User-Agent`, `Option` tolerates missing fields

Run: `cargo run -- torvalds` (first run downloads dependencies, needs internet)
