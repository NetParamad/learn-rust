# Learn Rust / เรียน Rust

## ภาษาไทย
Repo ทบทวน Rust แยกโฟลเดอร์ละเรื่อง เน้นตัวอย่างรันได้สั้นๆ (`01` ถึง `15` ใช้ stdlib เท่านั้น)

- `01-variables` ถึง `15-product-search-cli`: เข้าโฟลเดอร์แล้วรันด้วย `rustc --edition 2021 main.rs -o main && ./main` (Windows ใช้ `.\main.exe`)
- `16-weather-cli`, `17-github-profile-viewer`: เป็น Cargo project (ต้องมีเน็ตครั้งแรกเพื่อโหลด dependency) รันด้วย `cargo run -- "Bangkok"`

เป้าหมาย: `ownership` + `borrowing` + `error handling` ดูที่โฟลเดอร์ `10`, `11`, `12`

โปรเจคใช้งานจริง:
- `13-mini-project-text-stats` — นับบรรทัด/คำ/ตัวอักษร (CLI)
- `14-todo-cli` — Todo + ไฟล์ `todos.txt` (แทน localStorage)
- `15-product-search-cli` — ค้นหา + กรองหมวด (in-memory)
- `16-weather-cli` — เช็คอากาศรายเมือง (Open-Meteo ไม่ต้องใช้ API key)
- `17-github-profile-viewer` — ดูโปรไฟล์ GitHub (API)

## English
Rust review repo, one folder per topic, short runnable examples (`01` to `15` use stdlib only).

- `01-variables` to `15-product-search-cli`: enter the folder and run with `rustc --edition 2021 main.rs -o main && ./main` (Windows: `.\main.exe`)
- `16-weather-cli`, `17-github-profile-viewer`: Cargo projects (need internet once to fetch dependencies), run with `cargo run -- "Bangkok"`

Goal: `ownership` + `borrowing` + `error handling`, see folders `10`, `11`, `12`

Real-world projects:
- `13-mini-project-text-stats` — line/word/char counter (CLI)
- `14-todo-cli` — Todo + `todos.txt` file (instead of localStorage)
- `15-product-search-cli` — search + category filter (in-memory)
- `16-weather-cli` — city weather (Open-Meteo, no API key needed)
- `17-github-profile-viewer` — GitHub profile (API)
