use aho_corasick::{AhoCorasick, AhoCorasickBuilder, AhoCorasickKind};
use minify_js::Session;
use once_cell::sync::Lazy;

use crate::{
    cfg::Cfg,
    err::ProcessingResult,
    proc::{MatchAction::*, MatchMode::*, Processor},
};

static SCRIPT_END: Lazy<AhoCorasick> = Lazy::new(|| {
    AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .kind(Some(AhoCorasickKind::DFA))
        .build(["</script"])
        .unwrap()
});

// Provide `None` to `mode` if not JS.
#[inline(always)]
pub fn process_script(
    proc: &mut Processor,
    cfg: &Cfg,
    mode: Option<minify_js::TopLevelMode>,
) -> ProcessingResult<()> {
    proc.require_not_at_end()?;
    let src = proc.m(WhileNotSeq(&SCRIPT_END), Discard);
    // `process_tag` will require closing tag.

    if cfg.minify_js && mode.is_some() {
        // TODO Write to `out` directly, but only if we can guarantee that the length will never exceed the input.
        let mut output = Vec::new();
        let session = Session::new();
        let result = minify_js::minify(&session, mode.unwrap(), &proc[src], &mut output);
        // TODO Collect error as warning.
        if result.is_ok() && output.len() < src.len() {
            proc.write_slice(output.as_slice());
        } else {
            proc.write_range(src);
        };
    } else {
        proc.write_range(src);
    };

    Ok(())
}
