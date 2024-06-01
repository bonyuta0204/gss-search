use clap::builder::Str;
// src/fuzzy_finder.rs
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub struct FuzzyFinder {
    matcher: SkimMatcherV2,
}

impl FuzzyFinder {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
        }
    }

    pub fn search<'a>(&self, query: &str, data: &'a Vec<String>) -> Vec<&'a str> {
        let mut matched: Vec<_> = data
            .iter()
            .map(|item| (item, self.matcher.fuzzy_match(&item, query)))
            .filter_map(|(item, score)| score.map(|s| (item, s)))
            .collect();

        matched.sort_by(|a, b| b.1.cmp(&a.1));

        let result: Vec<&str> = matched.into_iter().map(|(item, _)| item.as_str()).collect();
        result
    }
}
