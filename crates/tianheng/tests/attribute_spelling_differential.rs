//! A **generated** differential over attribute spellings, with rustc as the third party.
//!
//! **Why this exists, and what it replaces.** Every defect this family has had in its attribute readers was
//! found by a person reading one more spelling, and each repair was correct and followed by another shape.
//! `BACKLOG.md`'s `path_meta_values` entry states the cost — nine wrong answers across two crates, every one
//! a clean verdict over source no build compiles or a violation against source the governed tree does not
//! have — and states what the null option cannot buy: *any statement about what remains*. The corpus is
//! every lexical form Rust admits, which no inspection enumerates.
//!
//! **Three parties, because two are not enough.** A differential over the three dimensions alone answers
//! *do they agree*, and agreement is not correctness: the raw bare-`cfg` spelling was missed by all three at
//! once, and a reader comparing only the three would have reported that as clean. So each shape carries a
//! **declared** answer, and rustc is asked whether that declaration is true:
//!
//! 1. **Is the spelling legal Rust?** Every shape is compiled with no cross-module reference. A failure
//!    here is the **generator's**: it claimed a shape Rust does not admit.
//! 2. **Does the remap apply?** The same source, plus a reference to an item defined only in the named
//!    target. For a governed shape under a live predicate this must resolve; for a look-alike its live
//!    probe must **not**, either by rustc refusing the attribute outright or by compiling without it.
//!
//! **The two questions were one arm, and the arm asserted the wrong fact.** A single compile carrying the
//! reference fails both when the spelling is illegal and when the spelling is fine and the *declared
//! answer* is wrong, and the message named only the first — measured, `#[cfg_attr(unix, allow(dead_code))]`
//! declared governed reported *rustc rejects the generated spelling … the corpus claims a shape Rust does
//! not admit*, over source rustc accepts. Two facts an author repairs in opposite places, so two questions.
//!
//! **What rustc cannot decide here, said rather than left to be assumed.** Under a false predicate no
//! configuration on this host compiles the target, so rustc can say only that the source is legal. The
//! declaration carries those, and the true-predicate rows are what hold the declaration to something. The
//! dimensions are cfg-blind by construction — they union every candidate a build *could* compile through —
//! so a false predicate is expected to be governed exactly as a true one is, and that property is asserted
//! rather than assumed.
//!
//! **Gated behind `TIANHENG_SPELLING_DIFFERENTIAL`**, because it compiles one crate per shape, and named in
//! the Definition of Done and in CI on its own line — the shape `pin_bites` already sets for a direction
//! that must not make the ordinary suite pay for it, and that must not be left to whoever remembers.

#[path = "support/mod.rs"]
mod support;
use support::{TempFixture, guibiao_exit, hunyi_exit, louke_exit};

const REASON: &str = "differential: one spelling, one answer, in every dimension";
const SEAM: &str = "conformance-seam";
const FORBIDDEN: &str = "pub mod forbidden { pub struct Thing; }\n";
/// A probe for the declared seam at top level, so 漏刻's declared-but-unprobed direction never fires as an
/// incidental confound: the only finding a fixture can produce is the one inside the file under test.
const PROBED: &str = "pub fn probed(o: u8) { assert_boundary!(\"conformance-seam\", o); }\n";
/// All three dimensions' violations in one file, plus the item rustc is asked to resolve.
const VIOLATIONS: &str = "use crate::forbidden::Thing;\n\
                          pub fn leak() -> crate::forbidden::Thing { crate::forbidden::Thing }\n\
                          pub fn typo(o: u8) { assert_boundary!(\"conformance-saem\", o); }\n\
                          pub fn only_in_target() -> u8 { 7 }\n";
const CLEAN: &str = "pub fn conventional() -> u8 { 0 }\n";

/// What the generator declares about a shape, and what rustc is asked to confirm.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Answer {
    /// `target.rs` is a remap candidate: every dimension governs it, so the violation inside reacts.
    Governed,
    /// The spelling looks like a remap and names nothing: no configuration compiles `target.rs`, so a
    /// dimension governing it is over-reading — a violation against source the tree does not have.
    Decoy,
}

struct Shape {
    label: String,
    attribute: String,
    answer: Answer,
    /// Whether the predicate holds on this host, and so whether rustc can be asked which file the build
    /// contains rather than only whether the source is legal.
    live_predicate: bool,
}

/// The generated corpus: wrapper × predicate for the governed half, and the look-alikes for the other.
///
/// A decoy is paired with a **false** predicate deliberately. `foo::path` under a live predicate is a path
/// rustc resolves and rejects, so the pairing is what makes the shape legal source at all — and the
/// compile step below is what would catch the pairing being wrong.
fn corpus() -> Vec<Shape> {
    let wrappers: [(&str, &str); 3] = [
        ("plain", "cfg_attr({pred}, {meta})"),
        ("nested", "cfg_attr({pred}, cfg_attr({pred}, {meta}))"),
        (
            "raw wrapper",
            "cfg_attr({pred}, r#cfg_attr({pred}, {meta}))",
        ),
    ];
    let predicates: [(&str, &str, bool); 3] = [
        ("unix", "unix", cfg!(unix)),
        (
            "compound",
            "all(unix, not(target_os = \"none\"))",
            cfg!(unix),
        ),
        ("false", "any()", false),
    ];
    let metas: [(&str, &str); 4] = [
        ("path", "path = \"target.rs\""),
        ("raw path", "r#path = \"target.rs\""),
        ("comment before name", "/*c*/ path = \"target.rs\""),
        ("comment before eq", "path /*c*/ = \"target.rs\""),
    ];

    let mut out = Vec::new();
    for (wl, wrapper) in wrappers {
        for (pl, pred, live) in predicates {
            for (ml, meta) in metas {
                out.push(Shape {
                    label: format!("{wl} · {pl} · {ml}"),
                    attribute: format!(
                        "#[{}]",
                        wrapper.replace("{pred}", pred).replace("{meta}", meta)
                    ),
                    answer: Answer::Governed,
                    live_predicate: live,
                });
            }
        }
    }
    // The look-alikes. Each names `target.rs` in a position no build compiles it from.
    for (label, meta) in [
        ("qualified path", "foo::path = \"target.rs\""),
        ("raw qualified path", "foo::r#path = \"target.rs\""),
        ("path in a non-cfg_attr group", "foo(path = \"target.rs\")"),
        (
            "path inside a string literal",
            "doc = \"path = \\\"target.rs\\\"\"",
        ),
    ] {
        out.push(Shape {
            label: format!("decoy · {label}"),
            attribute: format!("#[cfg_attr(any(), {meta})]"),
            answer: Answer::Decoy,
            live_predicate: false,
        });
    }
    for (label, attribute) in [
        (
            "qualified wrapper",
            "#[cfg_attr(any(), foo::cfg_attr(unix, path = \"target.rs\"))]",
        ),
        (
            "path key in a compound predicate",
            "#[cfg_attr(all(unix, path = \"target.rs\"), allow(dead_code))]",
        ),
        (
            "path before the first comma",
            "#[cfg_attr(path = \"target.rs\", allow(dead_code))]",
        ),
    ] {
        out.push(Shape {
            label: format!("decoy · {label}"),
            attribute: attribute.to_string(),
            answer: Answer::Decoy,
            live_predicate: false,
        });
    }
    out
}

/// Compile `source` as a crate and hand back rustc's own first line on failure.
fn compiles(name: &str, source: &str) -> Result<(), String> {
    let dir = std::env::temp_dir().join(format!("tianheng-spelling-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    xingbiao::claim_scratch(&dir).expect("the rustc scratch root is writable");
    let out = dir.join("out");
    std::fs::create_dir_all(&out).expect("create out dir");
    std::fs::write(
        dir.join("target.rs"),
        "pub fn only_in_target() -> u8 { 7 }\n",
    )
    .expect("write");
    std::fs::write(dir.join("imp.rs"), CLEAN).expect("write");
    let lib = dir.join("lib.rs");
    std::fs::write(&lib, source).expect("write");

    let run = std::process::Command::new("rustc")
        .args(["--crate-type", "lib", "--edition", "2021"])
        .arg(&lib)
        .arg("--out-dir")
        .arg(&out)
        .output()
        .map_err(|err| format!("rustc could not be run: {err}"))?;
    let verdict = if run.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&run.stderr)
            .lines()
            .next()
            .unwrap_or("(no stderr)")
            .to_string())
    };
    let _ = std::fs::remove_dir_all(&dir);
    verdict
}

/// The shape with no cross-module reference: the source whose legality is the generator's claim.
fn plain(attribute: &str) -> String {
    format!("{attribute}\npub mod imp;\n")
}

/// The same source, resolving an item defined **only** in the named target — so success is rustc saying
/// the remap applied and the build contains that file.
fn referencing_the_target(attribute: &str) -> String {
    format!("{attribute}\npub mod imp;\npub fn use_it() -> u8 {{ imp::only_in_target() }}\n")
}

/// A look-alike's live probe: the same spelling with its false predicate made true, so *no remap applies*
/// is a fact rustc can be asked rather than one the generator asserts. A shape carrying no `any()` is
/// already live — its predicate is the look-alike — and is probed as written.
fn live_probe(attribute: &str) -> String {
    attribute.replace("any()", "unix")
}

/// Every generated spelling is legal Rust, and every dimension answers it the way the generator declares —
/// which is also the way the other two answer it.
#[test]
fn every_generated_spelling_is_answered_the_same_way_by_every_dimension() {
    if std::env::var_os("TIANHENG_SPELLING_DIFFERENTIAL").is_none() {
        println!(
            "spelling differential: skipped — set TIANHENG_SPELLING_DIFFERENTIAL=1 to run it. It compiles \
             one crate per shape, so it is named on its own line in the Definition of Done and in CI \
             rather than made part of the ordinary suite."
        );
        return;
    }

    let mut offences: Vec<String> = Vec::new();
    let corpus = corpus();
    for (i, shape) in corpus.iter().enumerate() {
        let name = format!("diff-{i:02}");

        // **Question one, and the generator is what it holds.** A shape rustc will not take is not a
        // spelling a maintainer could write, and reporting the dimensions' answers about it would attribute
        // the generator's defect to them. No cross-module reference here, so this failure can mean only
        // one thing.
        if let Err(stderr) = compiles(&format!("{name}-plain"), &plain(&shape.attribute)) {
            offences.push(format!(
                "{}: rustc will not take the generated spelling `{}`, so the corpus claims a shape Rust \
                 does not admit — {stderr}",
                shape.label, shape.attribute
            ));
            continue;
        }

        // **Question two: is the declared answer true of a real build?** The same source, resolving an item
        // defined only in the named target. A failure here is the *declaration*, not the spelling — the
        // two were one arm and the arm named only the first.
        match shape.answer {
            Answer::Governed if shape.live_predicate => {
                if let Err(stderr) = compiles(
                    &format!("{name}-ref"),
                    &referencing_the_target(&shape.attribute),
                ) {
                    offences.push(format!(
                        "{}: rustc takes `{}` but the build does not contain the named target, so the \
                         corpus declares a remap that does not apply — {stderr}",
                        shape.label, shape.attribute
                    ));
                    continue;
                }
            }
            Answer::Decoy => {
                // Measured rather than asserted: with the predicate made live, the look-alike must still
                // name no remap — either rustc refuses the attribute outright, or it compiles and the
                // target's item does not resolve. Both are *no remap applies*; a success would mean the
                // generator called a real remap a decoy.
                let probe = live_probe(&shape.attribute);
                if compiles(&format!("{name}-decoy"), &referencing_the_target(&probe)).is_ok() {
                    offences.push(format!(
                        "{}: with its predicate made live, `{probe}` DOES apply a remap, so the corpus \
                         calls a real module target a look-alike",
                        shape.label
                    ));
                    continue;
                }
            }
            // A governed shape under a dead predicate: rustc on this host compiles no configuration that
            // reaches the target, so question one is all it can answer and the declaration carries the
            // rest. The live rows above are what keep that declaration honest.
            Answer::Governed => {}
        }

        let lib = format!("{FORBIDDEN}{PROBED}{}\npub mod imp;\n", shape.attribute);
        let fixture = TempFixture::new(&name, &lib);
        let src = fixture.lib().parent().expect("lib.rs has a parent");
        std::fs::write(src.join("imp.rs"), CLEAN).expect("write the conventional file");
        std::fs::write(src.join("target.rs"), VIOLATIONS).expect("write the named target");

        let expected = match shape.answer {
            Answer::Governed => 1,
            Answer::Decoy => 0,
        };
        let answers = [
            (
                "圭表",
                guibiao_exit(&name, fixture.manifest(), "crate::imp", REASON),
            ),
            (
                "渾儀",
                hunyi_exit(&name, fixture.manifest(), "crate::imp", REASON),
            ),
            ("漏刻", louke_exit(fixture.lib(), SEAM, REASON)),
        ];
        for (dimension, got) in answers {
            if got != expected {
                offences.push(format!(
                    "{}: {dimension} answered {got} where the spelling `{}` declares {expected} ({})",
                    shape.label,
                    shape.attribute,
                    match shape.answer {
                        Answer::Governed =>
                            "the named target is a candidate every dimension governs, so the violation \
                             inside it reacts",
                        Answer::Decoy =>
                            "no configuration compiles the named file, so governing it reports a violation \
                             against source the tree does not have",
                    }
                ));
            }
        }
    }

    assert!(
        offences.is_empty(),
        "{} of {} generated spellings were answered wrongly:\n  {}",
        offences.len(),
        corpus.len(),
        offences.join("\n  ")
    );
    let (governed, decoys) = corpus
        .iter()
        .fold((0, 0), |(g, d), shape| match shape.answer {
            Answer::Governed => (g + 1, d),
            Answer::Decoy => (g, d + 1),
        });
    println!(
        "spelling differential: {} spellings — {governed} governed and {decoys} look-alikes. Every one is \
         legal Rust; every look-alike was shown to apply no remap with its predicate made live; and every \
         dimension answered every one the way the corpus declares.",
        corpus.len()
    );
}
