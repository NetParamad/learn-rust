// 08 - structs โน้ต: struct + impl method, ..base ก็อป field ที่เหลือ
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
}

impl User {
    // constructor นิยมชื่อ new (Rust ไม่มี class/new keyword)
    fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    // &self คือ this: ยืมอ่าน ไม่กิน ownership
    fn greet(&self) -> String {
        format!("hi, I'm {} (#{})", self.name, self.id)
    }
}

fn main() {
    let u = User::new(1, "Net");
    println!("{}", u.greet());
    println!("field ตรงๆ: {} {}", u.id, u.name);

    // struct update syntax (แทน spread {...obj})
    let admin = User { id: 2, ..u.clone() };
    println!("{admin:?}");
}
