// 17 - github profile viewer: cargo run -- torvalds
// โน้ต: fetch 2 ต่อ (profile -> repos), เช็ค 404, GitHub บังคับ User-Agent
use serde::Deserialize;

#[derive(Deserialize)]
struct Profile {
    login: String,
    name: Option<String>,
    followers: u64,
    public_repos: u64,
}

#[derive(Deserialize)]
struct Repo {
    name: String,
    stargazers_count: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("torvalds"));

    // GitHub API บังคับส่ง User-Agent ไม่งั้นโดน 403
    let client = reqwest::blocking::Client::builder()
        .user_agent("learn-rust")
        .build()?;
    let base = format!("https://api.github.com/users/{user}");

    // ต่อที่ 1: โปรไฟล์ (เช็ค 404 ก่อนแบบ res.ok ของ JS)
    let res = client.get(&base).send()?;
    if res.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(format!("user '{user}' not found (404)").into());
    }
    let p: Profile = res.error_for_status()?.json()?;
    println!(
        "{} (@{})",
        p.name.as_deref().unwrap_or(&p.login),
        p.login
    );
    println!("followers: {}, public repos: {}", p.followers, p.public_repos);

    // ต่อที่ 2: 5 repo อัปเดตล่าสุดพร้อมดาว
    let repos: Vec<Repo> = client
        .get(format!("{base}/repos"))
        .query(&[("per_page", "5"), ("sort", "updated")])
        .send()?
        .error_for_status()?
        .json()?;
    println!("latest {} repo(s):", repos.len());
    for r in &repos {
        println!("- {} ({} stars)", r.name, r.stargazers_count);
    }
    Ok(())
}
