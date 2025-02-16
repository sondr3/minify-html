use aho_corasick::{AhoCorasick, AhoCorasickBuilder, AhoCorasickKind};
use once_cell::sync::Lazy;

pub static TAG_TEXTAREA_END: Lazy<AhoCorasick> = Lazy::new(|| {
    AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .kind(Some(AhoCorasickKind::DFA))
        .build(["</textarea"])
        .unwrap()
});
pub static TAG_TITLE_END: Lazy<AhoCorasick> = Lazy::new(|| {
    AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .kind(Some(AhoCorasickKind::DFA))
        .build(["</title"])
        .unwrap()
});
