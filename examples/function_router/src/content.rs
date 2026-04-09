use serde::{Deserialize, Serialize};

use crate::generator::{Generated, Generator};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub seed: u32,
    pub name: String,
    pub keywords: Vec<String>,
    pub image_url: String,
}

impl Generated for Author {
    fn generate(r#gen: &mut Generator) -> Self {
        let name = r#gen.human_name();
        let keywords = r#gen.keywords();
        let image_url = r#gen.face_image_url((150, 150));
        Self {
            seed: r#gen.seed,
            name,
            keywords,
            image_url,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PostMeta {
    pub seed: u32,
    pub title: String,
    pub author: Author,
    pub keywords: Vec<String>,
    pub image_url: String,
}

impl Generated for PostMeta {
    fn generate(r#gen: &mut Generator) -> Self {
        let title = r#gen.title();
        let author = Author::generate_from_seed(r#gen.new_seed());
        let keywords = r#gen.keywords();
        let image_url = r#gen.image_url((300, 150), &keywords);

        Self {
            seed: r#gen.seed,
            title,
            author,
            keywords,
            image_url,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub meta: PostMeta,
    pub content: Vec<PostPart>,
}

impl Generated for Post {
    fn generate(r#gen: &mut Generator) -> Self {
        const PARTS_MIN: u32 = 1;
        const PARTS_MAX: u32 = 10;

        let meta = PostMeta::generate(r#gen);

        let n_parts = r#gen.range(PARTS_MIN, PARTS_MAX);
        let content = (0..n_parts).map(|_| PostPart::generate(r#gen)).collect();

        Self { meta, content }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PostPart {
    Section(Section),
    Quote(Quote),
}

impl Generated for PostPart {
    fn generate(r#gen: &mut Generator) -> Self {
        // Because we pass the same (already used) generator down,
        // the resulting `Section` and `Quote` aren't be reproducible with just the seed.
        // This doesn't matter here though, because we don't need it.
        if r#gen.chance(1, 10) {
            Self::Quote(Quote::generate(r#gen))
        } else {
            Self::Section(Section::generate(r#gen))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub title: String,
    pub paragraphs: Vec<String>,
    pub image_url: String,
}

impl Generated for Section {
    fn generate(r#gen: &mut Generator) -> Self {
        const PARAGRAPHS_MIN: u32 = 1;
        const PARAGRAPHS_MAX: u32 = 8;

        let title = r#gen.title();
        let n_paragraphs = r#gen.range(PARAGRAPHS_MIN, PARAGRAPHS_MAX);
        let paragraphs = (0..n_paragraphs).map(|_| r#gen.paragraph()).collect();
        let image_url = r#gen.image_url((200, 100), &[]);

        Self {
            title,
            paragraphs,
            image_url,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    pub author: Author,
    pub content: String,
}

impl Generated for Quote {
    fn generate(r#gen: &mut Generator) -> Self {
        // wouldn't it be funny if the author ended up quoting themselves?
        let author = Author::generate_from_seed(r#gen.new_seed());
        let content = r#gen.paragraph();
        Self { author, content }
    }
}
