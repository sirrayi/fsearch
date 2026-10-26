//! Generated queries over a real index: parsing and searching never panic.
//! (A glob with a letter past ASCII before `?` or `[` once did: a request
//! thread died, and the client got no answer.)
use fsearch::index::Index;
use fsearch::live::Live;
use fsearch::query::{Query, Searcher};

/// Filter words, glob and regex syntax, numbers, and text past ASCII.
#[rustfmt::skip]
const ATOMS: &[&str] = &[
    "ext:", "type:", "kind:", "in:", "size:", "mtime:", "re:", "path:", "grep:", "regex:", "sym:", "limit:", "git:", "*", "**", "?", "[", "]", "{",
    "}", ",", "!", "^", "$", "'", "/", "~", "~/", "..", "\\", "(", ")", "|", "+", "-", ":", " ", ">", "<", "=", "k", "m", "1", "0",
    "99999999999999999999", "1.5", "d", "y", "rs", "main", "src", "a", "é", "e\u{301}", "İ", "ß", "ﬁ", "日本", "\u{0}", "\t", "\u{feff}", "{a,b}",
    "{,}", "[!a]", "[a-", "[]", "(?i)", "\\d", "**/", "file", "dir", "link",
];

#[test]
fn generated_queries_never_panic() {
    let live = Live::new(Index::build(fsearch::walk::scan(b"/usr/share/zoneinfo", 4), 0, 0, b"/usr"));
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed as usize
    };
    let mut panicked = Vec::new();
    for _ in 0..5000 {
        let q: String = (0..1 + next() % 7).map(|_| ATOMS[next() % ATOMS.len()]).collect();
        let run = std::panic::catch_unwind(|| {
            if let Ok(parsed) = Query::parse(&q, "/usr") {
                Searcher { live: &live }.search(&parsed);
            }
        });
        if run.is_err() {
            panicked.push(q);
        }
    }
    assert!(panicked.is_empty(), "{} queries panicked, e.g. {:?}", panicked.len(), &panicked[..panicked.len().min(5)]);
}
