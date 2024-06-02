use std::ops::Deref;

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

    pub fn search<'a, S: ToString>(&self, query: &str, data: &'a Vec<S>) -> Vec<&'a S> {
        if query.len() == 0 {
            return data.iter().collect();
        }
        let mut matched: Vec<_> = data
            .iter()
            .map(|item| (item, self.matcher.fuzzy_match(&item.to_string(), query)))
            .filter_map(|(item, score)| score.map(|s| (item, s)))
            .collect();

        matched.sort_by(|a, b| b.1.cmp(&a.1));

        let result: Vec<_> = matched.into_iter().map(|(item, _)| item).collect();
        result
    }
}
