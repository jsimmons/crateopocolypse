const NUM_CRATES: usize = 100;

fn main() {
    //std::fs::remove_dir_all("testy").unwrap();
    std::fs::create_dir("testy").unwrap();

    let members_str = (0..NUM_CRATES)
        .map(|i| format!(r#""crate-{}""#, i))
        .collect::<Vec<_>>()
        .join(",");

    std::fs::write(
        "testy/Cargo.toml",
        format!(
            r#"[workspace]
members = [
    {}
]"#,
            members_str
        ),
    )
    .unwrap();

    let mut dep = None;
    for i in 0..NUM_CRATES {
        std::fs::create_dir_all(format!("testy/crate-{}/src", i)).unwrap();
        std::fs::write(
            format!("testy/crate-{}/src/lib.rs", i),
            r#"pub fn test() -> i32 { 4 }"#,
        )
        .unwrap();

        let dependency_line = if let Some(d) = dep {
            format!(r#"crate-{} = {{ path = "../crate-{}" }}"#, d, d)
        } else {
            String::new()
        };

        std::fs::write(
            format!("testy/crate-{}/Cargo.toml", i),
            format!(
                r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
{}"#,
                format!("crate-{}", i),
                dependency_line
            ),
        )
        .unwrap();

        dep = Some(i);
    }
}
