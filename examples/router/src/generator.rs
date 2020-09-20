use lipsum::MarkovChain;
use rand::{rngs::SmallRng, seq::IteratorRandom, Rng, SeedableRng};

const KEYWORDS: &str = include_str!("../data/keywords.txt");
const YEW_CONTENT: &str = include_str!("../data/yew.txt");

const KEYWORDS_MIN: usize = 1;
const KEYWORDS_MAX: usize = 4;

const TITLE_MIN_WORDS: usize = 3;
const TITLE_MAX_WORDS: usize = 8;
const TITLE_SMALL_WORD: usize = 3;

const SENTENCE_MIN_WORDS: usize = 7;
const SENTENCE_MAX_WORDS: usize = 25;

const PARAGRAPH_MIN_SENTENCES: usize = 3;
const PARAGRAPH_MAX_SENTENCES: usize = 10;

pub struct Generator<'a> {
    pub seed: u64,
    rng: SmallRng,
    // TODO: store a version of this in a global variable
    yew_chain: MarkovChain<'a, SmallRng>,
}
impl Generator<'static> {
    pub fn from_seed(seed: u64) -> Self {
        let rng = SmallRng::seed_from_u64(seed);
        let mut yew_chain = MarkovChain::new_with_rng(rng.clone());
        yew_chain.learn(YEW_CONTENT);

        Self {
            seed,
            rng,
            yew_chain,
        }
    }
}
impl Generator<'_> {
    pub fn new_seed(&mut self) -> u64 {
        self.rng.gen()
    }

    pub fn image_url(&mut self, dimension: (usize, usize), keywords: &[String]) -> String {
        let cache_buster = self.rng.gen::<u16>();
        let (width, height) = dimension;
        format!(
            "https://source.unsplash.com/random/{}x{}?{}#{}",
            width,
            height,
            keywords.join(","),
            cache_buster
        )
    }

    pub fn face_image_url(&mut self, dimension: (usize, usize)) -> String {
        self.image_url(dimension, &["human".to_owned(), "face".to_owned()])
    }

    pub fn human_name(&mut self) -> String {
        // TODO
        "Name in progress".to_owned()
    }

    pub fn keywords(&mut self) -> Vec<String> {
        let n_keywords = self.rng.gen_range(KEYWORDS_MIN, KEYWORDS_MAX);
        KEYWORDS
            .split_whitespace()
            .map(ToOwned::to_owned)
            .choose_multiple(&mut self.rng, n_keywords)
    }

    pub fn title(&mut self) -> String {
        let n_words = self.rng.gen_range(TITLE_MIN_WORDS, TITLE_MAX_WORDS);
        let mut title = String::new();

        let words = self
            .yew_chain
            .iter()
            .map(|word| word.trim_matches(|c: char| c.is_ascii_punctuation()))
            .filter(|word| !word.is_empty())
            .take(n_words);

        for (i, word) in words.enumerate() {
            if i > 0 {
                title.push(' ');
            }

            // Capitalize the first word and all long words.
            if i == 0 || word.len() > TITLE_SMALL_WORD {
                title.push_str(&naive_title_case(word));
            } else {
                title.push_str(word);
            }
        }
        title
    }

    pub fn sentence(&mut self) -> String {
        let n_words = self.rng.gen_range(SENTENCE_MIN_WORDS, SENTENCE_MAX_WORDS);
        self.yew_chain.generate(n_words)
    }

    pub fn paragraph(&mut self) -> String {
        let n_sentences = self
            .rng
            .gen_range(PARAGRAPH_MIN_SENTENCES, PARAGRAPH_MAX_SENTENCES);
        let mut paragraph = String::new();
        for i in 0..n_sentences {
            if i > 0 {
                paragraph.push(' ');
            }

            paragraph.push_str(&self.sentence());
        }
        paragraph
    }
}

fn naive_title_case(word: &str) -> String {
    let idx = match word.chars().next() {
        Some(c) => c.len_utf8(),
        None => 0,
    };

    let mut result = String::with_capacity(word.len());
    result.push_str(&word[..idx].to_uppercase());
    result.push_str(&word[idx..]);
    result
}

pub trait Generated: Sized {
    fn generate(gen: &mut Generator) -> Self;
    fn generate_from_seed(seed: u64) -> Self {
        Self::generate(&mut Generator::from_seed(seed))
    }
}
