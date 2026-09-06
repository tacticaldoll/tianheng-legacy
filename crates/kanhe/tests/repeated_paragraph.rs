//! Repository check: no comment paragraph in this repository's Rust is written twice in a row.
//!
//! A paste that lands twice compiles, formats, lints and reads as deliberate. Nothing in the toolchain has an
//! opinion about it, and the second copy is invisible to the person who made it for the same reason it was
//! made: the eye that skips a paragraph it has already read is the eye that pasted it.
//!
//! It matters here more than the missing newline this crate's sibling refuses, because of what this
//! repository's prose is **for**. The rules are carried by weight rather than by enforcement — what sits in
//! an agent's context is what gets imitated — and a paragraph appearing twice is that weight doubled by
//! accident. The defect this was written for stood in `release_coherence_gate`, where the paragraph naming
//! why presence is asked by `ls-tree` rather than `show` was six lines long and stood twice, byte-identical.
//!
//! **The reach is adjacency, and the corpus is Rust.** Both are stops this reader makes deliberately, so
//! both are declared rather than left to be inferred: `docs/observation-bounds.md` carries them as
//! `repository-checks/a-paragraph-repeated-out-of-line-is-not-read-a-stated-bound` and
//! `repository-checks/a-paragraph-repeated-in-prose-is-not-read-a-stated-bound`.

use std::path::{Path, PathBuf};

use kanhe::refusal::{Kind, Refusal, cannot_judge, violation};

fn workspace_root() -> Option<PathBuf> {
    shengmo::workspace::locate(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        |root| root.join("Cargo.toml").is_file(),
        shengmo::workspace::marker_set(),
    )
}

/// What a comment line says, or `None` where the line is not one.
///
/// `//`, `///` and `//!` are one class here: the question is whether a paragraph was written twice, and the
/// marker that introduces it decides who reads it, not whether it was pasted. A line whose content is empty
/// answers `Some("")` and is refused a place in a block below — see [`repetitions`].
fn comment_body(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    let rest = trimmed.strip_prefix("//")?;
    Some(rest.trim_start_matches(['/', '!']).trim())
}

/// Every comment paragraph in `text` that is immediately followed by a byte-identical copy of itself, as
/// `(line of the second copy, how many lines it spans)`.
///
/// **Longest first, and non-overlapping.** A twelve-line paragraph written twice also contains a six-line
/// half written twice; reporting the longest run at a position and resuming past both copies names the
/// paste once, the way the author made it.
///
/// **Every line of the block must carry content.** Without that, four consecutive bare `//` lines — a
/// paragraph break spelled twice, which is a formatting matter and not a paste — are a two-line block
/// repeated, and this check would report a file whose text is exactly what its author wrote.
fn repetitions(text: &str) -> Vec<(usize, usize)> {
    let lines: Vec<&str> = text.lines().collect();
    let total = lines.len();
    let mut found = Vec::new();
    let mut start = 0usize;

    while start < total {
        let longest = (total - start) / 2;
        let mut matched = None;
        for span in (2..=longest).rev() {
            let block = &lines[start..start + span];
            if block != &lines[start + span..start + 2 * span] {
                continue;
            }
            if block
                .iter()
                .any(|line| !matches!(comment_body(line), Some(body) if !body.is_empty()))
            {
                continue;
            }
            matched = Some(span);
            break;
        }
        match matched {
            Some(span) => {
                found.push((start + span + 1, span));
                start += 2 * span;
            }
            None => start += 1,
        }
    }

    found
}

/// Every repeated paragraph under `root` among the Rust files `listing` names, and how many were read.
///
/// The count is returned for the reason its sibling in `whitespace_hygiene` returns one: an empty offence
/// list is what a clean corpus and a corpus that collapsed to nothing both produce, and those are opposite
/// facts. Here the corpus can collapse two ways — the enumeration answering nothing, and the `.rs` filter
/// matching nothing — and neither is visible in a verdict.
fn offences(root: &Path, listing: &[String]) -> (Vec<Refusal>, usize) {
    let mut offences = Vec::new();
    let mut inspected = 0usize;

    for path_str in listing {
        if !path_str.ends_with(".rs") {
            continue;
        }
        let content = match std::fs::read(root.join(path_str)) {
            Ok(content) => content,
            Err(err) => {
                offences.push(cannot_judge(format!(
                    "{path_str} is tracked Rust and could not be read ({err}), so whether it repeats a \
                     paragraph is unknown — an unread file is not a file that repeats nothing"
                )));
                continue;
            }
        };
        let Ok(text) = String::from_utf8(content) else {
            offences.push(cannot_judge(format!(
                "{path_str} is tracked Rust and is not UTF-8, so its comments were never read. A lossy \
                 decode would compare text this repository does not hold"
            )));
            continue;
        };
        inspected += 1;

        for (line, span) in repetitions(&text) {
            offences.push(violation(format!(
                "{path_str}:{line}: a {span}-line comment paragraph is written twice in a row; the second \
                 copy begins here"
            )));
        }
    }

    (offences, inspected)
}

/// Every path `git` tracks, exactly as git spells it.
///
/// `-z` rather than a line-oriented read: git quotes a path carrying a special character, and a reader that
/// took the quoted spelling would open a file that is not there and report the tracked file it could not
/// read. The refusal would be honest and the corpus would still have lost the file.
///
/// **Through [`kanhe::hermetic_git::run`], which refuses bytes it cannot represent.** Spelled here with its
/// own `Command`, this took git's stdout through `from_utf8_lossy` — and that runner's own header names
/// **this command** as the reason it does not: `ls-files -z` promises nothing about encoding, so a tracked
/// path that is not UTF-8 arrives as a different path than the one on disk, and every read downstream is
/// made against that. The offence would then name an identity the repository does not hold, which is the
/// property `xingbiao::path_identity` exists to keep and `repository_path` refuses in the same words. A
/// verdict is not owed on an input this reader cannot represent; saying so is.
fn tracked(root: &Path) -> Vec<String> {
    match kanhe::hermetic_git::run(root, &[], &["ls-files", "-z"]) {
        Ok(listing) => listing
            .split('\0')
            .filter(|path| !path.is_empty())
            .map(str::to_string)
            .collect(),
        Err(failure) => panic!(
            "CannotJudge: `git ls-files -z` did not answer a path list this reader can hold ({failure:?}), \
             so no file was inspected — an enumeration that could not be read is not an empty repository"
        ),
    }
}

#[test]
fn no_tracked_rust_file_writes_a_comment_paragraph_twice() {
    let Some(root) = workspace_root() else {
        return;
    };

    let listing = tracked(&root);
    let (offences, inspected) = offences(&root, &listing);

    assert!(
        inspected > 0,
        "no tracked Rust file was inspected, so this check would report clean over nothing — the vacuity \
         direction. {} tracked path(s) were enumerated",
        listing.len()
    );

    assert!(
        offences.is_empty(),
        "{inspected} tracked Rust file(s) inspected; a comment paragraph is written twice, or a tracked \
         file could not be read:\n{}",
        offences
            .iter()
            .map(|refusal| format!("  {:?}: {}", refusal.kind, refusal.message))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// The paragraph this check was written for is read, and the file around it is not.
///
/// The shape as it stood: a six-line comment paragraph, byte-identical, with ordinary source on both sides.
/// Supplied here rather than cited from history, because a direction resting on the live corpus stops
/// guarding the moment the corpus is repaired — the defect the citation reader's own scenario carried, and
/// the reason that one now supplies the spans it reads.
///
/// **The fixture is assembled, not written out.** This file is inside the corpus the live sweep reads, so a
/// paragraph spelled twice in these lines would be a finding against this file — measured, before it was
/// assembled: `crates/kanhe/tests/repeated_paragraph.rs:200: a 6-line comment paragraph is written twice`.
/// The subject is built from one paragraph repeated at run time, which leaves no two identical adjacent
/// lines in the source and keeps the fixture exact.
#[test]
fn a_paragraph_pasted_twice_is_read_and_its_neighbours_are_not() {
    let paragraph = "\
    // **Presence is asked first, by a command whose exit status answers it.** `show HEAD:` exits `128`
    // for a path that is not in HEAD *and* for a tree it cannot read, so a single `Err` arm had to choose
    // one meaning for both — and choosing *not a snapshot* classified a broken object store as a cycle.
    // Measured: `ls-tree HEAD -- <path>` exits `0` with an empty listing when the path is absent, `0` with
    // a line when it is there, and `128` only when the tree cannot be read. The question decides the
    // command, which is what the sibling tag-presence reader already does for the same shape.
";
    let around = |body: &str| format!("fn judge() {{\n{body}    let listed = run();\n}}\n");

    let repeated = around(&paragraph.repeat(2));
    assert_eq!(
        repetitions(&repeated),
        vec![(8, 6)],
        "the six-line paste must be read once, at the line its second copy begins"
    );

    // The control: the same file with one copy is silent, so the reading above is about the repetition
    // rather than about a check that reports whatever it is shown.
    let single = around(paragraph);
    assert_ne!(single, repeated, "the control must differ from the subject");
    assert_eq!(
        repetitions(&single),
        Vec::new(),
        "one copy of the paragraph is not a repetition"
    );
}

/// A paragraph break written twice is formatting, not a paste.
///
/// The narrowing that keeps this check from reporting text its author wrote: without the content
/// requirement, four bare `//` lines are a two-line block repeated.
#[test]
fn consecutive_empty_comment_lines_are_not_a_repetition() {
    assert_eq!(repetitions("//\n//\n//\n//\n"), Vec::new());
    assert_eq!(repetitions("/// a\n//\n//\n//\n//\n/// b\n"), Vec::new());
}

/// Repeated code is not this check's question, and neither is a repetition split by code.
///
/// Both shapes are live in this repository and both are deliberate: `runner/tests.rs` passes
/// `--manifest-path` twice to assert the duplicate flag exits `2`, and `hunyi`'s test helper spells the same
/// parameter types twice in one function type. A reader keyed on identical lines rather than on identical
/// *comment* lines reports them, which is the tax that would make this check something to work around.
#[test]
fn identical_code_lines_are_not_read() {
    assert_eq!(
        repetitions(
            "    \"--manifest-path\",\n    &fixture(\"clean\"),\n    \"--manifest-path\",\n    &fixture(\"clean\"),\n"
        ),
        Vec::new()
    );
    assert_eq!(
        repetitions(
            "    // a note\n    // and its second line\n    let x = 1;\n    // a note\n    // and its second line\n"
        ),
        Vec::new(),
        "a repetition that is not adjacent is outside this check's declared reach"
    );
}

/// A prose file is not this check's corpus, and the reason is not that prose repeats less.
///
/// Markdown repeats identical adjacent lines for its own reasons — a table's rule row, two list items that
/// happen to read the same, a fenced block quoted beside the text that explains it — and none of them is a
/// paste. Reading `.md` here would buy a class this repository has never met with a false positive it meets
/// routinely, which is the trade `PROJECT.md` refuses. The stop is declared rather than assumed, and this
/// direction is where it is pinned: the same duplicated paragraph that fires in a `.rs` file is silent in a
/// `.md` one, and the file is reported as uninspected rather than as clean.
#[test]
fn a_repeated_paragraph_in_a_prose_file_is_outside_the_corpus() {
    let scratch = std::env::temp_dir().join(format!(
        "tianheng-repeated-paragraph-prose-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&scratch);
    xingbiao::claim_scratch(&scratch).expect("the scratch root is writable");

    let body = "// a paragraph worth pasting\n// and its second line\n// a paragraph worth pasting\n// and its second line\n";
    std::fs::write(scratch.join("probe.rs"), body).expect("write the Rust probe");
    std::fs::write(scratch.join("probe.md"), body).expect("write the prose probe");

    let (rust, rust_inspected) = offences(&scratch, &["probe.rs".to_string()]);
    let (prose, prose_inspected) = offences(&scratch, &["probe.md".to_string()]);
    let _ = std::fs::remove_dir_all(&scratch);

    assert_eq!(rust_inspected, 1);
    assert_eq!(
        rust.len(),
        1,
        "the same text must fire inside the corpus: {rust:?}"
    );
    assert_eq!(rust[0].kind, Kind::Violation);

    assert_eq!(
        prose_inspected, 0,
        "a prose file is outside the corpus, so it is not counted as inspected — a stop, not a clean read"
    );
    assert!(prose.is_empty(), "and it is not reported: {prose:?}");
}

/// A tracked path that is not UTF-8 is refused, never renamed.
///
/// The property this file's enumeration stands on — held over [`tracked`] itself, not over the runner it
/// calls. Written against `hermetic_git::run` the direction passed with the defect restored, because the
/// defect was `tracked` reaching past that runner; a direction that observes a dependency cannot see its
/// caller stop using it.
///
/// `ls-files -z` hands back git's raw bytes, so an undecodable one arrives intact. A lossy decode
/// substitutes U+FFFD per byte and answers a path the repository does not hold, which is then opened,
/// missed, and reported as *a tracked file that could not be read* — a refusal that is honest about the
/// wrong file. The control is that measurement: git's spelling and the lossy one differ, so the file
/// reported would not be the file tracked.
#[cfg(unix)]
#[test]
fn a_tracked_path_that_is_not_utf8_is_refused_rather_than_renamed() {
    use std::os::unix::ffi::OsStrExt;

    let scratch = std::env::temp_dir().join(format!(
        "tianheng-repeated-paragraph-not-utf8-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&scratch);
    xingbiao::claim_scratch(&scratch).expect("the scratch root is writable");

    std::fs::write(scratch.join("plain.rs"), "fn main() {}\n").expect("write the decodable probe");
    for args in [
        &["init", "-q", "."][..],
        &["config", "user.email", "probe@example.invalid"][..],
        &["config", "user.name", "probe"][..],
        &["add", "-A"][..],
        &["commit", "-qm", "probe"][..],
    ] {
        kanhe::hermetic_git::run(&scratch, &[], args).expect("the fixture repository is built");
    }

    // The control first: with every path decodable, this enumeration answers them.
    let listed = tracked(&scratch);
    assert_eq!(
        listed,
        vec!["plain.rs".to_string()],
        "the enumeration reads a repository whose paths it can represent"
    );

    let undecodable = std::ffi::OsStr::from_bytes(b"probe-\xff.rs");
    std::fs::write(scratch.join(undecodable), "// a note\n// a note\n")
        .expect("write the undecodable probe");
    for args in [&["add", "-A"][..], &["commit", "-qm", "undecodable"][..]] {
        kanhe::hermetic_git::run(&scratch, &[], args).expect("the undecodable path is tracked");
    }

    let refused = std::panic::catch_unwind(|| tracked(&scratch));
    let _ = std::fs::remove_dir_all(&scratch);

    let payload = refused.expect_err(
        "a path this reader cannot represent must stop the enumeration, not be decoded into another name",
    );
    let said = payload
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| payload.downcast_ref::<&str>().copied())
        .unwrap_or("");
    assert!(
        said.contains("CannotJudge") && said.contains("ls-files"),
        "the refusal names what could not be read: {said:?}"
    );

    // And the measurement the refusal exists for: decoded lossily, the path is a name nothing tracks.
    let lossy = String::from_utf8_lossy(b"probe-\xff.rs").into_owned();
    assert!(
        lossy.contains('\u{fffd}') && lossy != "probe-\u{ff}.rs",
        "a lossy decode substitutes U+FFFD, so the name reported would not be the name tracked: {lossy:?}"
    );
}

/// An unreadable tracked file is a cannot-judge, not a file that repeats nothing.
#[test]
fn an_unreadable_tracked_rust_file_is_refused_rather_than_skipped() {
    let scratch = std::env::temp_dir().join(format!(
        "tianheng-repeated-paragraph-unreadable-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&scratch);
    xingbiao::claim_scratch(&scratch).expect("the scratch root is writable");

    let listing = vec!["zzz_absent_paragraph_probe.rs".to_string()];
    let (offences, inspected) = offences(&scratch, &listing);
    let _ = std::fs::remove_dir_all(&scratch);

    assert_eq!(offences.len(), 1, "{offences:?}");
    assert_eq!(
        offences[0].kind,
        Kind::CannotJudge,
        "an unread file is not a file without offences, so it is not a violation"
    );
    assert!(
        offences[0]
            .message
            .contains("zzz_absent_paragraph_probe.rs"),
        "the refusal must name the file it could not read, got {:?}",
        offences[0].message
    );
    assert_eq!(inspected, 0);
}
