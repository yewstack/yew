use lipsum::MarkovChain;
use once_cell::sync::Lazy;
use rand::distributions::Bernoulli;
use rand::rngs::SmallRng;
use rand::seq::IteratorRandom;
use rand::{Rng, SeedableRng};

const KEYWORDS: &str = include_str!("../data/keywords.txt");
const SYLLABLES: &str = include_str!("../data/syllables.txt");
const YEW_CONTENT: &str = include_str!("../data/yew.txt");

static YEW_CHAIN: Lazy<MarkovChain<'static>> = Lazy::new(|| {
    let mut chain = MarkovChain::new();
    chain.learn(YEW_CONTENT);
    chain
});

pub struct Generator {
    pub seed: u64,
    rng: SmallRng,
}
impl Generator {
    pub fn from_seed(seed: u64) -> Self {
        let rng = SmallRng::seed_from_u64(seed);

        Self { seed, rng }
    }
}
impl Generator {
    pub fn new_seed(&mut self) -> u64 {
        self.rng.gen()
    }

    /// [low, high)
    pub fn range(&mut self, low: usize, high: usize) -> usize {
        self.rng.gen_range(low..high)
    }

    /// `n / d` chance
    pub fn chance(&mut self, n: u32, d: u32) -> bool {
        self.rng.sample(Bernoulli::from_ratio(n, d).unwrap())
    }

    pub fn image_url(&mut self, dimension: (usize, usize), keywords: &[String]) -> String {
        let cache_buster = self.rng.gen::<u16>();
        let (width, height) = dimension;
        format!(
            "https://source.unsplash.com/random/{}x{}?{}&sig={}",
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
        const SYLLABLES_MIN: usize = 1;
        const SYLLABLES_MAX: usize = 5;

        let n_syllables = self.rng.gen_range(SYLLABLES_MIN..SYLLABLES_MAX);
        let first_name = SYLLABLES
            .split_whitespace()
            .choose_multiple(&mut self.rng, n_syllables)
            .join("");

        let n_syllables = self.rng.gen_range(SYLLABLES_MIN..SYLLABLES_MAX);
        let last_name = SYLLABLES
            .split_whitespace()
            .choose_multiple(&mut self.rng, n_syllables)
            .join("");

        format!("{} {}", title_case(&first_name), title_case(&last_name))
    }

    pub fn keywords(&mut self) -> Vec<String> {
        const KEYWORDS_MIN: usize = 1;
        const KEYWORDS_MAX: usize = 4;

        let n_keywords = self.rng.gen_range(KEYWORDS_MIN..KEYWORDS_MAX);
        KEYWORDS
            .split_whitespace()
            .map(ToOwned::to_owned)
            .choose_multiple(&mut self.rng, n_keywords)
    }

    pub fn title(&mut self) -> String {
        const WORDS_MIN: usize = 3;
        const WORDS_MAX: usize = 8;
        const SMALL_WORD_LEN: usize = 3;

        let n_words = self.rng.gen_range(WORDS_MIN..WORDS_MAX);
        let mut title = String::new();

        let words = YEW_CHAIN
            .iter_with_rng(&mut self.rng)
            .map(|word| word.trim_matches(|c: char| c.is_ascii_punctuation()))
            .filter(|word| !word.is_empty())
            .take(n_words);

        for (i, word) in words.enumerate() {
            if i > 0 {
                title.push(' ');
            }

            // Capitalize the first word and all long words.
            if i == 0 || word.len() > SMALL_WORD_LEN {
                title.push_str(&title_case(word));
            } else {
                title.push_str(word);
            }
        }
        title
    }

    pub fn sentence(&mut self) -> String {
        const WORDS_MIN: usize = 7;
        const WORDS_MAX: usize = 25;

        let n_words = self.rng.gen_range(WORDS_MIN..WORDS_MAX);
        YEW_CHAIN.generate_with_rng(&mut self.rng, n_words)
    }

    pub fn paragraph(&mut self) -> String {
        const SENTENCES_MIN: usize = 3;
        const SENTENCES_MAX: usize = 20;

        let n_sentences = self.rng.gen_range(SENTENCES_MIN..SENTENCES_MAX);
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

fn title_case(word: &str) -> String {
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
