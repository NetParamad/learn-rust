[English](README.md) | [ไทย](README.th.md)

# Learn Rust

รีโปทบทวน Rust แบบลงมือทำ แยกโฟลเดอร์ละเรื่องตามลำดับเลข แต่ละอันมีตัวอย่างสั้นๆ ที่รันได้ พร้อมโปรเจกต์ CLI ที่เอาแนวคิดต่างๆ มาประกอบกัน

## ภาพรวม

รีโปนี้เป็นเส้นทางการเรียน ไม่ใช่ไลบรารี โฟลเดอร์ `01`–`17` แต่ละอันเป็นบทเรียนที่สมบูรณ์ในตัว มี `main.rs` ของตัวเองและ `README.md` เล็กๆ ที่สรุปโน้ตของบทนั้น (สองภาษา ไทย/อังกฤษ) โฟลเดอร์ช่วงแรก (`01`–`15`) คอมไพล์ด้วย `rustc` ตรงๆ และใช้แค่ standard library ของ Rust เท่านั้น ส่วนสองอันสุดท้าย (`16`, `17`) เป็นโปรเจกต์ Cargo ที่ดึง `reqwest` กับ `serde` มาใช้คุยกับ HTTP API สาธารณะ

ตัวอย่างทั้งหมดเน้นเรื่องที่ทำให้ Rust ต่างจาก JavaScript/TypeScript นั่นคือ ownership, borrowing และ error handling พร้อมเทียบสำนวนจาก JS ให้เห็นว่าฝั่ง Rust เขียนยังไง

## สิ่งที่จะได้เรียน

### ตอนที่ 1 — พื้นฐานภาษา (`01`–`09`)

| # | บทเรียน | แนวคิดหลัก |
|---|---------|-------------|
| 01 | [Variables](01-variables) | `let` แก้ค่าไม่ได้ถ้าไม่ใส่ `mut`, `const` ต้องระบุ type และห้าม assign ใหม่, `shadowing` ประกาศชื่อซ้ำทับของเดิมได้ (เปลี่ยน type ได้ด้วย) |
| 02 | [Data Types](02-data-types) | `i32/u64/f64/bool/char`, แกะ `tuple` ด้วย destructuring, `array` ขนาดตายตัว, `String` เป็นเจ้าของข้อมูล vs `&str` แค่ยืมมอง |
| 03 | [Operators](03-operators) | หารจำนวนเต็มตัดเศษ, ไม่มี `++`/`--` (ใช้ `+= 1`), `==` เทียบเท่ากันตรงๆ ได้เลย (ไม่ต้องมี `===` แบบ JS) |
| 04 | [Conditions](04-conditions) | `if` เป็น expression คืนค่าได้ (แทน ternary), `match` แทน `switch` แต่บังคับครบทุกเคส (`_` คือ default) |
| 05 | [Loops](05-loops) | `loop` วนไม่รู้จบ (คืนค่าได้ด้วย `break ค่า`), `0..5` ไม่รวม 5 / `0..=5` รวม 5, `enumerate()` ให้ index ฟรี |
| 06 | [Functions](06-functions) | บรรทัดสุดท้ายที่ไม่มี `;` คือค่าที่ return, closure แทน arrow fn, ไม่มี default param (ใช้ `Option` + `unwrap_or`) |
| 07 | [Vectors](07-vectors) | `Vec` คือ array ที่โตได้, `vec![]` สร้างพร้อมค่า, index เกินแล้ว panic (ใช้ `.get()` ได้ `Option` แทน), copy ด้วย `.clone()` |
| 08 | [Structs](08-structs) | `struct` แทน object, `impl` ใส่ method (`&self` คือ `this`), `..base` ก็อป field ที่เหลือ (ต้อง `clone` เพราะ `String` ไม่ใช่ `Copy`) |
| 09 | [Iterators](09-iterators) | `map` แปลง, `filter` กรอง, `find` เอาตัวแรก, `fold`/`sum` รวมยอด — iterator ขี้เกียจ ต้อง `.collect()` ก่อนถึงจะทำงาน |

### ตอนที่ 2 — แก่นของ Rust (`10`–`12`)

| # | บทเรียน | แนวคิดหลัก |
|---|---------|-------------|
| 10 | [Ownership](10-ownership) | หัวใจของ Rust — `String` ย้ายเจ้าของ (`move`) แล้วตัวเก่าใช้ต่อไม่ได้, type เล็ก (`i32`) เป็น `Copy` เลยไม่ move, `.clone()` ก็อปลึกแบบชัดเจน |
| 11 | [Borrowing](11-borrowing) | `&` ยืมอ่านกี่ตัวก็ได้, `&mut` ยืมเขียนได้ตัวเดียวและห้ามมีตัวอื่นค้าง, ใช้เสร็จแล้ว borrow หมดอายุ (ยืมใหม่ได้) |
| 12 | [Error Handling](12-error-handling) | `Option` แทน null/`??` (`unwrap_or`), `Result` แทน throw (`match`/`?` กระจาย error ขึ้น caller), `.expect()` มี message แต่ panic ถ้าพัง |

### ตอนที่ 3 — โปรเจกต์ CLI ใช้งานจริง (`13`–`17`)

| # | โปรเจกต์ | ทำอะไร | จุดเด่น |
|---|----------|--------|----------|
| 13 | [mini-project-text-stats](13-mini-project-text-stats) | รับชื่อไฟล์ แล้วนับบรรทัด / คำ / ตัวอักษร (ไม่ใส่ arg จะนับข้อความตัวอย่างแทน) | `env::args`, `fs::read_to_string`, `lines().count()`, `split_whitespace`, `chars` |
| 14 | [todo-cli](14-todo-cli) | Todo เก็บลงไฟล์ `todos.txt` (แทน localStorage): เพิ่ม / ลิสต์ / ติ๊กเสร็จ / ลบ | `env::args`, `fs::read`/`write`, `split_once`, `filter_map` |
| 15 | [product-search-cli](15-product-search-cli) | ค้นหา + กรองหมวดแบบ in-memory: คำค้นกรองชื่อ, ใส่หมวดเป็น arg ตัวที่สองเพื่อกรอง category | `filter` + `contains`, ตัดหมวดซ้ำด้วย `contains` check, `to_lowercase` เทียบแบบไม่สนตัวพิมพ์ |
| 16 | [weather-cli](16-weather-cli) | พิมพ์ชื่อเมือง → แปลงเป็นพิกัด (Geocoding) → ดึงอากาศปัจจุบันจาก Open-Meteo (ไม่ต้องใช้ API key) | `reqwest::blocking` (ไม่ต้อง async), `.query()` encode URL ให้, `error_for_status`, `serde` แปลง JSON |
| 17 | [github-profile-viewer](17-github-profile-viewer) | พิมพ์ username ของ GitHub → ดู followers / จำนวน repo + 5 repo ที่อัปเดตล่าสุดพร้อมจำนวนดาว | ยิง request ต่อกัน 2 ครั้ง, เช็ค 404 ด้วย `error_for_status`, GitHub บังคับส่ง `User-Agent`, `Option` รับ field ที่ขาดได้ |

## เทคโนโลยีที่ใช้

- **ภาษา:** Rust (บท `01`–`15` ใช้แค่ standard library)
- **ตัวคอมไพล์:** ใช้ `rustc` ตรงๆ สำหรับ `01`–`15`; ใช้ Cargo สำหรับ `16`–`17`
- **Edition:** ใช้ `--edition 2021` ตอนเรียก `rustc` เอง; โปรเจกต์ Cargo ประกาศ `edition = "2024"`
- **crate ภายนอก (เฉพาะ `16`, `17`):**
  - `reqwest` `0.12` เปิด feature `blocking` และ `json` — HTTP client
  - `serde` `1` เปิด feature `derive` — แปลง JSON

## โครงสร้างโปรเจกต์

```
learn-rust/
├── 01-variables/                 # หนึ่งโฟลเดอร์ต่อหนึ่งเรื่อง
├── 02-data-types/
├── 03-operators/
├── 04-conditions/
├── 05-loops/
├── 06-functions/
├── 07-vectors/
├── 08-structs/
├── 09-iterators/
├── 10-ownership/
├── 11-borrowing/
├── 12-error-handling/
├── 13-mini-project-text-stats/   # ใช้ rustc ตรงๆ แต่ละอันมี README.md + main.rs
├── 14-todo-cli/
├── 15-product-search-cli/
├── 16-weather-cli/               # โปรเจกต์ Cargo
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/main.rs
├── 17-github-profile-viewer/     # โปรเจกต์ Cargo
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/main.rs
└── README.md
```

โฟลเดอร์บทเรียน `01`–`15` แต่ละอันมี `README.md` (โน้ตของเรื่องนั้น) กับ `main.rs` ไฟล์เดียว ส่วนโปรเจกต์ Cargo `16` และ `17` จะมี `Cargo.toml` / `Cargo.lock` และวางซอร์สไว้ที่ `src/main.rs`

## เริ่มต้นใช้งาน

### สิ่งที่ต้องมี

- Rust toolchain (`rustc` + `cargo`) — รีโปนี้พัฒนาและทดสอบด้วย `rustc 1.99.0` / `cargo 1.99.0`
- อินเทอร์เน็ตสำหรับการรันโปรเจกต์ Cargo (`16`, `17`) ครั้งแรก เพื่อโหลด dependency และตัวโปรแกรมก็เรียก API สาธารณะตอนรันด้วย

### รันตัวอย่างไฟล์เดียว (`01`–`15`)

เข้าไปในโฟลเดอร์บทเรียน แล้วคอมไพล์และรัน:

```bash
rustc --edition 2021 main.rs -o main && ./main
```

บน Windows:

```powershell
rustc --edition 2021 main.rs -o main && .\main.exe
```

### รันโปรเจกต์ Cargo (`16`–`17`)

จากในโฟลเดอร์ของโปรเจกต์:

```bash
cargo run -- "Bangkok"     # 16-weather-cli
cargo run -- torvalds      # 17-github-profile-viewer
```

คำสั่ง Cargo ที่ใช้บ่อย:

```bash
cargo build          # คอมไพล์โดยไม่รัน
cargo run -- <args>  # คอมไพล์แล้วรันพร้อม argument
cargo check          # เช็ค type/borrow แบบเร็ว
cargo fmt            # จัดรูปแบบโค้ด
cargo clippy         # ตรวจ lint
```

## วิธีใช้งาน

- **Text stats** — `./main main.rs` นับไฟล์ตัวมันเอง; `./main` นับข้อความตัวอย่างที่ฝังไว้
- **Todo** — `./main add "อ่าน Rust"` แล้ว `./main list`, `./main done 1`, `./main del 1` งานจะถูกเก็บใน `todos.txt` ในโฟลเดอร์ที่รัน และคงอยู่ข้ามการรัน
- **Product search** — `./main` แสดงทั้งหมด, `./main laptop` กรองตามชื่อ, `./main "" electronics` กรองเฉพาะหมวด
- **Weather** — `cargo run -- "Bangkok"` แสดงอุณหภูมิปัจจุบันและ weather code ของเมือง
- **GitHub viewer** — `cargo run -- torvalds` แสดงข้อมูลโปรไฟล์และ repo ล่าสุด

ถ้าไม่ใส่อาร์กิวเมนต์ `16` และ `17` จะใช้ค่าเริ่มต้นเป็น `"Bangkok"` และ `torvalds` ตามลำดับ

## หมายเหตุ

- บท `01`–`15` ใช้ **standard library เท่านั้น** ไม่ต้องใช้ Cargo และไม่ต้องต่อเน็ต
- โปรเจกต์ Cargo (`16`, `17`) ต้องต่อเน็ตครั้งแรกเพื่อดึง `reqwest` กับ `serde` หลังจากนั้นจะใช้แคชในเครื่อง
- ไม่ต้องใช้ API key หรือ credential ใดๆ เพราะ Open-Meteo และ endpoint เริ่มต้นของ GitHub API เป็นสาธารณะ ไม่ต้อง authenticate
- ไฟล์ binary ที่คอมไพล์แล้ว (`main` / `main.exe`), โฟลเดอร์ `target/` ของ Cargo และ `todos.txt` ที่สร้างขึ้นถูก gitignore ไว้ ไม่ได้ commit เข้ารีโป
- คอมเมนต์ในซอร์สและโน้ตใน README ของแต่ละบทเขียนเป็นภาษาไทย โดยแต่ละโฟลเดอร์ก็มีสรุปเป็นภาษาอังกฤษให้ด้วย
