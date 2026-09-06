//! Repository check: a `git` this repository constructs itself is the builder's, or it is declared.
//!
//! `kanhe::hermetic_git::hermetic` decides what a `git` behind a verdict may inherit — the configuration
//! files, the `GIT_CONFIG_*` channels, and `GIT_DIR`/`GIT_WORK_TREE`/`GIT_INDEX_FILE`, which move **which
//! repository** the command acts on and so reach past `current_dir` entirely. A caller that constructs its
//! own `Command::new("git")` inherits all of it.
//!
//! **The class this closes is not a bare invocation; it is a copy that inherits nothing.** The enumeration
//! `hermetic_git::tracked_records` owns states three properties a caller must not decide for itself. Two
//! sites cannot reach that owner — `shengmo`'s test targets, because `kanhe` depends on `shengmo` and the
//! edge would close a cycle — so they hold the properties by transcription, and transcription is partial by
//! nature: the first pass carried `-z` and the strict decode across and left the isolation behind, in both
//! copies, unmentioned in either comment. Nothing said what a copy owes, so nothing noticed two of three.
//!
//! This is the sibling shape of `gate_exit_classes`: membership in a declared set is what it holds, and the
//! purpose beside each entry is a reader's aid rather than a fact this check judges. What it adds is the
//! second column — whether the site claims the isolation — because a boundary-forced copy that claims it and
//! does not carry it is exactly the state this was written after.
//!
//! **The granularity is the file**, like its sibling's. A file declared `Isolated` must carry every
//! environment operation the builder makes; that a construction and those operations sit in the same
//! function is not checked, and no declared file holds more than one construction today.

use std::collections::BTreeSet;
use std::path::PathBuf;

/// What a declared site does about the environment a bare `git` inherits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Isolation {
    /// The builder itself, or a direction whose subject **is** what an un-isolated `git` answers.
    Exempt,
    /// A site that cannot reach the builder and holds its environment properties by hand.
    Isolated,
}

/// Every file that constructs a `git` without the builder, why, and what it does about the environment.
///
/// A site that gains one must be named here. A name that outlives its site must go: the comparison below is
/// two-directional for the reason `repository-checks` states for every constant a check judges by — a
/// one-directional comparison catches an omission and misses an entry that has outlived its subject.
const CONSTRUCTS_GIT_ITSELF: [(&str, Isolation, &str); 3] = [
    (
        "crates/kanhe/src/tests/hermetic_git.rs",
        Isolation::Exempt,
        "the builder's own directions, whose subject is what an un-isolated `git` answers — an isolated \
         control could not measure the difference the builder makes",
    ),
    (
        "crates/shengmo/tests/family_coverage.rs",
        Isolation::Isolated,
        "boundary-forced: `shengmo` cannot reach `kanhe`, since `kanhe` depends on `shengmo` and the edge \
         would close a cycle",
    ),
    (
        "crates/shengmo/tests/examples_suite.rs",
        Isolation::Isolated,
        "boundary-forced, for the same reason as its sibling above",
    ),
];

/// The environment operations the builder makes, which a site declaring [`Isolation::Isolated`] must make too.
///
/// Held against `kanhe::hermetic_git`'s own text rather than written out from memory, so a variable the
/// builder starts clearing is one this check starts requiring.
const ENVIRONMENT_OPERATIONS: [&str; 11] = [
    "GIT_CONFIG_GLOBAL",
    "GIT_CONFIG_SYSTEM",
    "GIT_CONFIG_NOSYSTEM",
    "GIT_CONFIG_COUNT",
    "GIT_CONFIG_KEY_0",
    "GIT_CONFIG_VALUE_0",
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_CONFIG_PARAMETERS",
    "GIT_CONFIG",
];

fn workspace_root() -> Option<PathBuf> {
    shengmo::workspace::locate(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        |root| root.join("crates/kanhe/src/hermetic_git.rs").is_file(),
        shengmo::workspace::marker_set(),
    )
}

/// Whether `line` constructs a `git` rather than mentioning one.
///
/// A line whose trimmed text opens a comment is prose: this repository's own documentation names the shape
/// it forbids, four times, and a reader that counted those would refuse the sentences explaining the rule.
/// That is a stop this check makes deliberately — a construction inside a string literal is not read either,
/// and `repeated_paragraph` records what deciding that properly costs.
///
/// **The literal form, and the builder is not written in it.** `hermetic` takes its program as a parameter,
/// so `Command::new(program)` is what it spells and this reader does not see it — which is correct here,
/// since the builder is the construction everything else is routed *through* rather than one to declare.
/// The stop that leaves is declared: `repository-checks/a-git-constructed-through-a-program-value-is-not-read-a-stated-bound`.
fn constructs_git(line: &str) -> bool {
    let trimmed = line.trim_start();
    !trimmed.starts_with("//") && trimmed.contains("Command::new(\"git\")")
}

#[test]
fn every_git_this_repository_constructs_is_the_builders_or_is_declared() {
    let Some(root) = workspace_root() else {
        return;
    };

    let tracked = kanhe::hermetic_git::tracked_paths(&root, &["*.rs"]).expect(
        "the tracked Rust is enumerable; a failed enumeration is not a repository with no sources",
    );
    let mut constructing = BTreeSet::new();
    let mut examined = 0usize;
    for path in &tracked {
        let text = std::fs::read_to_string(root.join(path)).unwrap_or_else(|err| {
            panic!("cannot read tracked file '{path}' — a file this check claims to have inspected must have been read: {err}")
        });
        examined += 1;
        if text.lines().any(constructs_git) {
            constructing.insert(path.clone());
        }
    }
    assert!(
        examined > 0,
        "no tracked Rust was inspected, so this check would report clean over nothing"
    );

    let declared: BTreeSet<String> = CONSTRUCTS_GIT_ITSELF
        .iter()
        .map(|(path, _, _)| (*path).to_string())
        .collect();
    assert_eq!(
        declared, constructing,
        "the files constructing a `git` differ from the set named here. A site that gains one must be \
         named with why; a name that outlives its site must go. {examined} tracked Rust file(s) were read"
    );
}

#[test]
fn a_site_that_cannot_reach_the_builder_holds_what_the_builder_holds() {
    let Some(root) = workspace_root() else {
        return;
    };

    // The requirement is taken from the builder rather than from this file's memory of it: an operation the
    // builder starts making is one a declared copy starts owing.
    let builder = std::fs::read_to_string(root.join("crates/kanhe/src/hermetic_git.rs"))
        .expect("the builder is readable");
    let builder_code: Vec<&str> = builder
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect();
    for variable in ENVIRONMENT_OPERATIONS {
        assert!(
            builder_code.iter().any(|line| line.contains(variable)),
            "{variable} is required of every declared copy and the builder no longer names it, so this \
             check requires something its own subject does not do"
        );
    }

    let mut missing = Vec::new();
    let mut checked = 0usize;
    for (path, isolation, _) in CONSTRUCTS_GIT_ITSELF {
        if isolation != Isolation::Isolated {
            continue;
        }
        checked += 1;
        let text = std::fs::read_to_string(root.join(path))
            .unwrap_or_else(|err| panic!("cannot read the declared site '{path}': {err}"));
        // **Code, not the comment beside it.** Written against the whole text this direction passed with the
        // three `env_remove` calls deleted, because the paragraph explaining why they are there names every
        // variable it removes — a reader counting its own explanation, which is the class
        // `repeated_paragraph` met from the other side.
        let code: Vec<&str> = text
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect();
        for variable in ENVIRONMENT_OPERATIONS {
            if !code.iter().any(|line| line.contains(variable)) {
                missing.push(format!("  {path}: does not handle {variable}"));
            }
        }
    }
    assert!(
        checked > 0,
        "no declared site claims the isolation, so this direction would hold over nothing — the state it \
         was written for is a copy that claims it and does not carry it"
    );
    assert!(
        missing.is_empty(),
        "a site declared to hold the builder's environment properties does not hold all of them, which is \
         the partial-transcription class this check exists for:\n{}",
        missing.join("\n")
    );
}

/// A `git` mentioned in prose is not a `git` constructed.
///
/// The stop above, pinned: this repository's own documentation names `Command::new("git")` in four places to
/// explain the rule, and a reader counting those would refuse the sentences that state it.
#[test]
fn a_construction_named_in_prose_is_not_read() {
    assert!(!constructs_git(
        "/// `Command::new(\"git\")` is what this forbids"
    ));
    assert!(!constructs_git(
        "    // a bare Command::new(\"git\") inherits the environment"
    ));
    assert!(constructs_git("    let out = Command::new(\"git\")"));
    assert!(constructs_git("Command::new(\"git\")"));
}

/// The declared set names a path this repository tracks.
///
/// A declaration pointing at a file that no longer exists is an entry nobody can falsify: the membership
/// comparison above would report it as missing from the constructing set and read as a site that stopped
/// constructing, rather than as a name that outlived its subject.
#[test]
fn every_declared_site_is_a_tracked_path() {
    let Some(root) = workspace_root() else {
        return;
    };
    let tracked: BTreeSet<String> = kanhe::hermetic_git::tracked_paths(&root, &[])
        .expect("the tracked set is enumerable")
        .into_iter()
        .collect();
    for (path, _, _) in CONSTRUCTS_GIT_ITSELF {
        assert!(
            tracked.contains(path),
            "the declared site {path} is not tracked"
        );
    }
}

/// Every declared site carries a purpose, so the next reader is told why rather than left to infer it.
#[test]
fn every_declared_site_says_why() {
    for (path, _, why) in CONSTRUCTS_GIT_ITSELF {
        assert!(
            why.len() > 30,
            "the entry for {path} states no purpose a reader could act on"
        );
    }
}
