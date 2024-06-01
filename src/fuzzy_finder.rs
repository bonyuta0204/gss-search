// src/fuzzy_finder.rs

use fuzzy_matcher::clangd::ClangdMatcher;
use fuzzy_matcher::FuzzyMatcher;

pub struct FuzzyFinder {
    matcher: ClangdMatcher,
}

impl FuzzyFinder {
    pub fn new() -> Self {
        Self {
            matcher: ClangdMatcher::default(),
        }
    }

    pub fn search(&self, query: &str, data: &[Vec<String>]) -> Vec<&Vec<String>> {
        data.iter()
            .filter(|item| self.matcher.fuzzy_match(&item.join(" "), query).is_some())
            .collect()
    }
}
