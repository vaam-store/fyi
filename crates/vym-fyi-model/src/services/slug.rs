use rand::seq::IndexedRandom;
use rand::Rng;

const ADJECTIVES: &[&str] = &[
    "happy", "swift", "bright", "calm", "eager", "gentle", "bold", "keen", "warm", "cool", "wild",
    "soft", "quick", "pure", "smart", "proud", "brave", "fair", "fine", "gold",
];

const NOUNS: &[&str] = &[
    "cat", "fox", "owl", "bear", "wolf", "deer", "hawk", "swan", "lion", "tree", "moon", "sun",
    "star", "wave", "bird", "fish", "leaf", "rose", "oak", "gold",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SlugGenerationMode {
    #[default]
    Human,
    Hex,
}

impl SlugGenerationMode {
    pub fn from_env() -> Self {
        match std::env::var("SLUG_GENERATION_MODE")
            .as_deref()
            .map(str::to_lowercase)
            .as_deref()
        {
            Ok("hex") => SlugGenerationMode::Hex,
            Ok("human") | Err(_) => SlugGenerationMode::Human,
            _ => SlugGenerationMode::Human,
        }
    }
}

pub fn generate_slug(min_len: usize) -> String {
    let mode = SlugGenerationMode::from_env();
    match mode {
        SlugGenerationMode::Human => generate_human_slug(),
        SlugGenerationMode::Hex => generate_hex_slug(min_len),
    }
}

pub fn generate_human_slug() -> String {
    let mut rng = rand::rng();
    let adj = ADJECTIVES.choose(&mut rng).unwrap_or(&"quick");
    let noun = NOUNS.choose(&mut rng).unwrap_or(&"fox");
    let num: u32 = rng.random_range(1..100);
    format!("{}-{}-{}", adj, noun, num)
}

fn generate_hex_slug(min_len: usize) -> String {
    let min_len = min_len.max(6);
    let bytes_len = min_len.div_ceil(2);
    let mut buf = vec![0u8; bytes_len];
    rand::rng().fill(buf.as_mut_slice());

    let mut slug = String::with_capacity(bytes_len * 2);
    for b in buf {
        slug.push_str(&format!("{:02x}", b));
    }
    slug
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_slug_format() {
        for _ in 0..100 {
            let slug = generate_human_slug();
            let parts: Vec<&str> = slug.split('-').collect();
            assert_eq!(parts.len(), 3, "slug '{}' should have 3 parts", slug);
            assert!(
                ADJECTIVES.contains(&parts[0]),
                "adjective '{}' not in list",
                parts[0]
            );
            assert!(NOUNS.contains(&parts[1]), "noun '{}' not in list", parts[1]);
            let num: u32 = parts[2].parse().expect("number part should be numeric");
            assert!((1..=99).contains(&num), "number {} out of range", num);
        }
    }

    #[test]
    fn human_slug_lowercase() {
        for _ in 0..50 {
            let slug = generate_human_slug();
            assert_eq!(
                slug,
                slug.to_lowercase(),
                "slug '{}' should be lowercase",
                slug
            );
        }
    }

    #[test]
    fn hex_slug_format() {
        let slug = generate_hex_slug(6);
        assert!(
            slug.len() >= 6,
            "slug '{}' should be at least 6 chars",
            slug
        );
        assert!(
            slug.chars().all(|c| c.is_ascii_hexdigit()),
            "slug '{}' should be hex",
            slug
        );
    }

    #[test]
    fn slug_generation_mode_default() {
        let mode = SlugGenerationMode::from_env();
        assert_eq!(mode, SlugGenerationMode::Human);
    }
}
