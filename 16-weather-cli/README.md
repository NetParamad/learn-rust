# 16 Weather CLI

## ภาษาไทย
พิมพ์ชื่อเมือง -> แปลงเป็นพิกัด (Geocoding) -> ดึงอากาศปัจจุบัน (Open-Meteo ไม่ต้องใช้ API key)

โน้ต: `reqwest::blocking` (ไม่ต้อง async), `.query()` encode URL ให้, `error_for_status` แทนเช็ค `res.ok`, `serde` แปลง JSON

รัน: `cargo run -- "Bangkok"` (ครั้งแรกโหลด dependency ต้องมีเน็ต)

## English
Type a city -> geocode to coordinates -> fetch current weather (Open-Meteo, no API key needed).

Note: `reqwest::blocking` (no async needed), `.query()` URL-encodes for you, `error_for_status` replaces `res.ok` check, `serde` parses JSON

Run: `cargo run -- "Bangkok"` (first run downloads dependencies, needs internet)
