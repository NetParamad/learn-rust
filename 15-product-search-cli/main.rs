// 15 - product search CLI: ./main [query] [category]
use std::env;

struct Product {
    name: &'static str,
    category: &'static str,
    price: f64,
}

fn main() {
    let products = vec![
        Product { name: "Laptop Pro", category: "electronics", price: 999.0 },
        Product { name: "Phone Mini", category: "electronics", price: 499.0 },
        Product { name: "Rust Book", category: "books", price: 39.0 },
        Product { name: "Go Book", category: "books", price: 35.0 },
        Product { name: "T-Shirt", category: "clothes", price: 19.0 },
        Product { name: "Jeans", category: "clothes", price: 59.0 },
    ];

    // โชว์หมวดทั้งหมด (ตัดซ้ำแบบ Set ของ JS)
    let mut cats: Vec<&str> = Vec::new();
    for p in &products {
        if !cats.contains(&p.category) {
            cats.push(p.category);
        }
    }
    println!("categories: {}", cats.join(", "));

    let args: Vec<String> = env::args().collect();
    let query = args.get(1).map(|s| s.to_lowercase()).unwrap_or_default();
    let cat = args.get(2).map(|s| s.to_lowercase()).unwrap_or_default();

    let hits: Vec<&Product> = products
        .iter()
        .filter(|p| {
            (query.is_empty() || p.name.to_lowercase().contains(query.as_str()))
                && (cat.is_empty() || p.category == cat.as_str())
        })
        .collect();

    println!("found {} item(s):", hits.len());
    for p in hits {
        println!("- {} [{}] ${:.2}", p.name, p.category, p.price);
    }
}
