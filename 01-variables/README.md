# 01 Variables

## ภาษาไทย
โน้ต: ไม่ใส่ `mut` แก้ค่าไม่ได้, `const` ระบุ type + ห้าม assign ใหม่, `shadowing` ประกาศชื่อซ้ำทับของเดิมได้ (เปลี่ยน type ได้ด้วย)

รัน: `rustc --edition 2021 main.rs -o main && ./main`

## English
Note: without `mut` you cannot reassign, `const` needs a type + cannot be reassigned, `shadowing` re-declares the same name (can even change type)

Run: `rustc --edition 2021 main.rs -o main && ./main`
