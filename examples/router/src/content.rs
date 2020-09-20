use crate::generator::{Generated, Generator};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Author {
    pub seed: u64,
    pub name: String,
    pub image_url: String,
}
impl Generated for Author {
    fn generate(gen: &mut Generator) -> Self {
        let name = gen.human_name();
        let image_url = gen.face_image_url((600, 600));
        Self {
            seed: gen.seed,
            name,
            image_url,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Section {
    content: String,
    image_url: Option<String>,
}
impl Generated for Section {
    fn generate(gen: &mut Generator) -> Self {
        todo!()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Post {
    pub seed: u64,
    pub title: String,
    pub author: Author,
    pub keywords: Vec<String>,
    pub content: String,
    pub image_url: String,
}
impl Generated for Post {
    fn generate(gen: &mut Generator) -> Self {
        let title = gen.title();
        let author = Author::generate_from_seed(gen.new_seed());
        let keywords = gen.keywords();
        let content = gen.paragraph();
        let image_url = gen.image_url((1000, 500), &keywords);

        Self {
            seed: gen.seed,
            title,
            author,
            keywords,
            content,
            image_url,
        }
    }
}
