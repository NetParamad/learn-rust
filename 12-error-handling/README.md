# 12 Error Handling

## ภาษาไทย
โน้ต: `Option` แทน null/`??` (`unwrap_or`), `Result` แทน throw (`match`/`?` กระจาย error ขึ้น caller), `.expect()` มี message แต่ panic ถ้าพัง

รัน: `rustc --edition 2021 main.rs -o main && ./main`

## English
Note: `Option` replaces null/`??` (`unwrap_or`), `Result` replaces throw (`match`/`?` propagates to caller), `.expect()` has a message but panics on failure

Run: `rustc --edition 2021 main.rs -o main && ./main`
