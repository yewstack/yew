use crate::generator::{Generated, Generator};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Author {
    pub seed: u32,
    pub name: String,
    pub keywords: Vec<String>,
    pub image_url: String,
}

impl Generated for Author {
    fn generate(gen: &mut Generator) -> Self {
        let name = gen.human_name();
        let keywords = gen.keywords();
        let image_url = gen.face_image_url((600, 600));
        Self {
            seed: gen.seed,
            name,
            keywords,
            image_url,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostMeta {
    pub seed: u32,
    pub title: String,
    pub author: Author,
    pub keywords: Vec<String>,
    pub image_url: String,
}

impl Generated for PostMeta {
    fn generate(gen: &mut Generator) -> Self {
        let title = gen.title();
        let author = Author::generate_from_seed(gen.new_seed());
        let keywords = gen.keywords();
        let image_url = gen.image_url((1000, 500), &keywords);

        Self {
            seed: gen.seed,
            title,
            author,
            keywords,
            image_url,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Post {
    pub meta: PostMeta,
    pub content: Vec<PostPart>,
}

impl Generated for Post {
    fn generate(gen: &mut Generator) -> Self {
        const PARTS_MIN: u32 = 1;
        const PARTS_MAX: u32 = 10;

        let meta = PostMeta::generate(gen);

        let n_parts = gen.range(PARTS_MIN, PARTS_MAX);
        let content = (0..n_parts).map(|_| PostPart::generate(gen)).collect();

        Self { meta, content }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PostPart {
    Section(Section),
    Quote(Quote),
}

impl Generated for PostPart {
    fn generate(gen: &mut Generator) -> Self {
        // Because we pass the same (already used) generator down,
        // the resulting `Section` and `Quote` aren't be reproducible with just the seed.
        // This doesn't matter here though, because we don't need it.
        if gen.chance(1, 10) {
            Self::Quote(Quote::generate(gen))
        } else {
            Self::Section(Section::generate(gen))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Section {
    pub title: String,
    pub paragraphs: Vec<String>,
    pub image_url: String,
}

impl Generated for Section {
    fn generate(gen: &mut Generator) -> Self {
        const PARAGRAPHS_MIN: u32 = 1;
        const PARAGRAPHS_MAX: u32 = 8;

        let title = gen.title();
        let n_paragraphs = gen.range(PARAGRAPHS_MIN, PARAGRAPHS_MAX);
        let paragraphs = (0..n_paragraphs).map(|_| gen.paragraph()).collect();
        let image_url = gen.image_url((600, 300), &[]);

        Self {
            title,
            paragraphs,
            image_url,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Quote {
    pub author: Author,
    pub content: String,
}

impl Generated for Quote {
    fn generate(gen: &mut Generator) -> Self {
        // wouldn't it be funny if the author ended up quoting themselves?
        let author = Author::generate_from_seed(gen.new_seed());
        let content = gen.paragraph();
        Self { author, content }
    }
}
