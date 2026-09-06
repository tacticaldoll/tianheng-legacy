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

/// Every string literal a `.env(…)` or `.env_remove(…)` call in `stream` names.
///
/// **The operation, not the name appearing somewhere.** Held as `line.contains(variable)` over non-comment
/// text, this check was satisfied by `let _ = "GIT_DIR";` — an inert string standing in for an isolation
/// that is not made. It asked whether a *name* occurs where the property is whether a *call* happens, which
/// is the third round in a row this file read Rust as text for a question about code.
fn environment_calls(stream: proc_macro2::TokenStream) -> BTreeSet<String> {
    let mut named = BTreeSet::new();
    let mut pending = vec![stream];
    while let Some(stream) = pending.pop() {
        let trees: Vec<proc_macro2::TokenTree> = stream.into_iter().collect();
        for (index, tree) in trees.iter().enumerate() {
            if let proc_macro2::TokenTree::Group(group) = tree {
                pending.push(group.stream());
            }
            let proc_macro2::TokenTree::Ident(method) = tree else {
                continue;
            };
            if (method != "env" && method != "env_remove") || index == 0 || index + 1 >= trees.len()
            {
                continue;
            }
            let called = matches!(&trees[index - 1], proc_macro2::TokenTree::Punct(dot) if dot.as_char() == '.');
            let proc_macro2::TokenTree::Group(arguments) = &trees[index + 1] else {
                continue;
            };
            if !called || arguments.delimiter() != proc_macro2::Delimiter::Parenthesis {
                continue;
            }
            // Spelled without a `let` chain: this crate's MSRV predates them, and the workspace toolchain
            // compiles one silently — the MSRV job is what said so.
            if let Some(proc_macro2::TokenTree::Literal(variable)) =
                arguments.stream().into_iter().next()
            {
                if let Some(name) = variable
                    .to_string()
                    .strip_prefix('"')
                    .and_then(|rest| rest.strip_suffix('"'))
                {
                    named.insert(name.to_string());
                }
            }
        }
    }
    named
}

/// Every string literal in `stream`, for the two arrays [`kanhe::hermetic_git::hermetic`] iterates.
fn string_literals(stream: proc_macro2::TokenStream) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut pending = vec![stream];
    while let Some(stream) = pending.pop() {
        for tree in stream {
            match tree {
                proc_macro2::TokenTree::Group(group) => pending.push(group.stream()),
                proc_macro2::TokenTree::Literal(literal) => {
                    if let Some(name) = literal
                        .to_string()
                        .strip_prefix('"')
                        .and_then(|rest| rest.strip_suffix('"'))
                    {
                        found.insert(name.to_string());
                    }
                }
                _ => {}
            }
        }
    }
    found
}

/// The token group that is `name`'s body — a `fn`'s braces, or a `const`'s value after its `=`.
///
/// Named by its opening rather than sliced to a terminator. Searched for `"\n}\n"` with `"];\n"` as a
/// fallback, the two single-line `const` openings took the *next function's* closing brace — an 87-line span
/// for a one-line subject — and the fallback was a branch no input could reach. A group carries its own end,
/// so there is no terminator to pick and nothing to get wrong.
fn item_body(
    stream: proc_macro2::TokenStream,
    keyword: &str,
    name: &str,
) -> Option<proc_macro2::TokenStream> {
    let trees: Vec<proc_macro2::TokenTree> = stream.into_iter().collect();
    for index in 0..trees.len().saturating_sub(1) {
        let opens = matches!(&trees[index], proc_macro2::TokenTree::Ident(word) if word == keyword)
            && matches!(&trees[index + 1], proc_macro2::TokenTree::Ident(word) if word == name);
        if !opens {
            continue;
        }
        // A `fn`'s body is its first brace group; a `const`'s value is the group after its `=`, which keeps
        // the type's own brackets and the doc attribute's out.
        let mut cursor = index + 2;
        if keyword == "const" {
            while cursor < trees.len()
                && !matches!(&trees[cursor], proc_macro2::TokenTree::Punct(equals) if equals.as_char() == '=')
            {
                cursor += 1;
            }
        }
        while cursor < trees.len() {
            if let proc_macro2::TokenTree::Group(group) = &trees[cursor] {
                let wanted = if keyword == "const" {
                    proc_macro2::Delimiter::Bracket
                } else {
                    proc_macro2::Delimiter::Brace
                };
                if group.delimiter() == wanted {
                    return Some(group.stream());
                }
            }
            cursor += 1;
        }
    }
    None
}

/// Every `GIT_*` variable [`kanhe::hermetic_git::hermetic`] acts on, read from the builder's own items.
///
/// `hermetic`'s own body for the calls it makes, and the two arrays it iterates for the names it removes.
/// The fixture-side `commit` names `GIT_AUTHOR_DATE` and `GIT_COMMITTER_DATE`, which are no part of what a
/// *read* inherits and are outside these items by construction rather than by a filter over a wider span.
fn environment_operations_of(builder: &str) -> BTreeSet<String> {
    let stream: proc_macro2::TokenStream = builder
        .parse()
        .expect("the builder is Rust this reader can tokenise");
    let body = item_body(stream.clone(), "fn", "hermetic").expect(
        "the builder no longer holds a `fn hermetic`, so this reader's subject is not its subject",
    );
    let mut found = environment_calls(body);
    let mut arrays = 0usize;
    for array in ["CONFIG_CHANNELS", "REPOSITORY_SELECTORS"] {
        let Some(value) = item_body(stream.clone(), "const", array) else {
            continue;
        };
        arrays += 1;
        found.extend(string_literals(value));
    }
    assert_eq!(
        arrays, 2,
        "the builder no longer holds both arrays `hermetic` iterates, so this reader would derive a set \
         from whichever half it found"
    );
    found
}

fn workspace_root() -> Option<PathBuf> {
    shengmo::workspace::locate(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        |root| root.join("crates/kanhe/src/hermetic_git.rs").is_file(),
        shengmo::workspace::marker_set(),
    )
}

/// Whether `text` constructs a `git` — read as tokens, because that is what the question is about.
///
/// **Three rounds of findings in this file were one cause: it read Rust as lines.** A substring over a
/// trimmed line missed a construction rustfmt split across lines, and read one inside a `/* … */` block —
/// contradicting the requirement in one direction and the prose bound in the other, at once. Comments are
/// what a lexer discards and a line break is not a token, so both go away by asking `proc_macro2` instead
/// of by adding an arm per shape. It is already a dev-dependency here, and `repeated_paragraph` took the
/// same route for the same reason.
///
/// The shape sought is `Command :: new ( "git" )` — the `Command` segment included, so a `new` on something
/// else is not read, and the argument compared as a **literal token** rather than as text, so any spelling
/// of the same string that a lexer produces is the same answer.
fn constructs_git(text: &str) -> bool {
    let Ok(stream) = text.parse::<proc_macro2::TokenStream>() else {
        // A file this reader cannot tokenise is one it cannot classify. Reporting it as constructing is the
        // over-reporting direction, which is visible; the corpus check reports how many it read either way.
        return text.contains("Command::new(\"git\")");
    };
    let mut pending = vec![stream];
    while let Some(stream) = pending.pop() {
        let trees: Vec<proc_macro2::TokenTree> = stream.into_iter().collect();
        for (index, tree) in trees.iter().enumerate() {
            if let proc_macro2::TokenTree::Group(group) = tree {
                pending.push(group.stream());
            }
            let proc_macro2::TokenTree::Ident(ident) = tree else {
                continue;
            };
            if ident != "new" || index < 3 || index + 1 >= trees.len() {
                continue;
            }
            let named_command = matches!(&trees[index - 3], proc_macro2::TokenTree::Ident(owner) if owner == "Command")
                && matches!(&trees[index - 2], proc_macro2::TokenTree::Punct(colon) if colon.as_char() == ':')
                && matches!(&trees[index - 1], proc_macro2::TokenTree::Punct(colon) if colon.as_char() == ':');
            if !named_command {
                continue;
            }
            let proc_macro2::TokenTree::Group(arguments) = &trees[index + 1] else {
                continue;
            };
            if arguments.delimiter() != proc_macro2::Delimiter::Parenthesis {
                continue;
            }
            // A trailing comma is what rustfmt leaves when it wraps the argument, so the group is the
            // literal and optionally that comma — the shape, not the spelling.
            let inner: Vec<proc_macro2::TokenTree> = arguments.stream().into_iter().collect();
            let program = match inner.as_slice() {
                [proc_macro2::TokenTree::Literal(program)] => Some(program),
                [
                    proc_macro2::TokenTree::Literal(program),
                    proc_macro2::TokenTree::Punct(comma),
                ] if comma.as_char() == ',' => Some(program),
                _ => None,
            };
            if program.is_some_and(|program| program.to_string() == "\"git\"") {
                return true;
            }
        }
    }
    false
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
        if constructs_git(&text) {
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

    // **Both directions, which the first spelling of this claimed and did not do.** Compared one way it
    // caught the builder dropping a variable and never the builder gaining one: a twelfth `env_remove` in
    // `hermetic` would be required of no copy, and both boundary-forced copies would fall silently behind —
    // this file's own constant carrying the class the file exists to close. The scope is `hermetic`'s own
    // span plus the two arrays it iterates, not the whole file: `commit` names `GIT_AUTHOR_DATE` and
    // `GIT_COMMITTER_DATE` for the fixture side, and those are no part of what a read inherits.
    let builder = std::fs::read_to_string(root.join("crates/kanhe/src/hermetic_git.rs"))
        .expect("the builder is readable");
    let makes: BTreeSet<String> = environment_operations_of(&builder);
    let required: BTreeSet<String> = ENVIRONMENT_OPERATIONS
        .iter()
        .map(|v| (*v).to_string())
        .collect();
    assert_eq!(
        makes, required,
        "the environment operations `hermetic` makes differ from the set this check requires of a copy. \
         An operation the builder gains is one a copy owes, and a name that outlives the builder must go"
    );

    let mut missing = Vec::new();
    let mut checked = 0usize;
    for (path, isolation, _) in CONSTRUCTS_GIT_ITSELF {
        if isolation != Isolation::Isolated {
            continue;
        }
        checked += 1;
        let text = std::fs::read_to_string(root.join(path))
            .unwrap_or_else(|err| panic!("cannot read the declared site '{path}': {err}"));
        // **The call, not the name occurring somewhere.** Written as `line.contains(variable)` over
        // non-comment text, this was satisfied by `let _ = "GIT_DIR";` — an inert string standing in for an
        // isolation that is not made. The line filter it carried was itself a repair of the same shape one
        // round earlier, when the paragraph explaining the isolation named every variable it removes and so
        // satisfied the check on its own. Both go away by reading the calls.
        let stream: proc_macro2::TokenStream = text
            .parse()
            .unwrap_or_else(|err| panic!("the declared site '{path}' is not tokenisable: {err}"));
        let made = environment_calls(stream);
        for variable in ENVIRONMENT_OPERATIONS {
            if !made.contains(variable) {
                missing.push(format!(
                    "  {path}: makes no `env`/`env_remove` call naming {variable}"
                ));
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
/// One of the two stops [`constructs_git`] makes, each now pinned by a direction named for it. They were one
/// test, and the bound for the **program-value** stop cited this name — a pin that resolves and exercises a
/// different stop, which is the class repaired one round earlier for
/// `a-paragraph-repeated-out-of-line-is-not-read`, in the round that repaired it.
#[test]
fn a_construction_named_in_prose_is_not_read() {
    assert!(!constructs_git(
        "/// `Command::new(\"git\")` is what this forbids\nfn f() {}"
    ));
    assert!(!constructs_git(
        "fn f() {\n    // a bare Command::new(\"git\") inherits the environment\n}"
    ));
    // **Every comment form, which is what asking a lexer buys.** A trimmed-line reader saw `//` and not
    // `/* … */`, so a block comment carrying the spelling was read as a construction — the prose bound
    // saying the opposite two files away.
    assert!(!constructs_git(
        "fn f() {\n    /* a bare Command::new(\"git\") inherits the environment */\n}"
    ));
    assert!(!constructs_git(
        "fn f() {\n    let x = 1; /* Command::new(\"git\") */\n}"
    ));
    assert!(constructs_git(
        "fn f() {\n    let out = Command::new(\"git\");\n}"
    ));
}

/// A construction rustfmt split across lines is still a construction.
///
/// The other half of what reading lines cost: a substring over one trimmed line saw neither the opening nor
/// the argument, so a file whose construction wrapped was **not** in the constructing set and the check's
/// own requirement — that every file constructing a `git` is declared — was wider than its reader.
#[test]
fn a_construction_split_across_lines_is_read() {
    assert!(constructs_git(
        "fn f() {\n    let out = Command::new(\n        \"git\",\n    );\n}"
    ));
    // The control: the same wrapping around a different program is not read, so the assertion above is
    // about the argument rather than about a reader that reports every `new`.
    assert!(!constructs_git(
        "fn f() {\n    let out = Command::new(\n        \"cargo\",\n    );\n}"
    ));
    assert!(!constructs_git(
        "fn f() {\n    let out = Other::new(\"git\");\n}"
    ));
}

/// A `git` constructed through a program value is not read.
///
/// The stop the bound of that name declares, pinned by a direction that exercises **it**. Whether a value
/// names `git` is not decidable from the line that constructs it, and the builder itself is written that way
/// — `Command::new(program)` — which is why the stop exists rather than being closed.
#[test]
fn a_construction_through_a_program_value_is_not_read() {
    assert!(!constructs_git(
        "    let mut command = Command::new(program);"
    ));
    assert!(!constructs_git("    let out = Command::new(&exe)"));
    // The control: the literal form on the same shape of line is read, so the assertions above are about
    // the value rather than about a reader that reports nothing.
    assert!(constructs_git(
        "    let mut command = Command::new(\"git\");"
    ));
}

/// A construction inside an ordinary string literal is not read.
///
/// The third stop, and the one that was mis-stated twice in a row: first as *a construction inside a string
/// literal is not read either*, which is false for a raw string; then, correcting that, as **not a stop at
/// all** — which threw away the half that is one. Both halves are real and they point opposite ways. This
/// one is the under-reaction, so it is the one declared:
/// `repository-checks/a-git-constructed-inside-a-string-literal-is-not-read-a-stated-bound`.
///
/// The mechanism is escaping. Rust source carrying a construction inside an ordinary literal spells it
/// `Command::new(\"git\")`, which is not the unescaped text this reader looks for, so the file drops out —
/// and a file that writes Rust and compiles it is where that matters. Deciding it properly means separating
/// a literal from the code around it, which `repeated_paragraph` carries a lexer to do and this check does
/// not.
///
/// **Both fixtures are assembled, and the raw one has to be.** This file is inside the corpus the live sweep
/// reads, so a raw string carrying the spelling verbatim would make this file report itself — measured, it
/// did. The alternatives were to declare this file exempt, which would blind the check to a real
/// construction added here later, or to leave the fixture written out and lose that. Assembling is not the
/// workaround `repeated_paragraph` records regretting: there it hid a defect being fixed, and here the
/// over-report is behaviour this direction exists to state.
#[test]
fn a_construction_inside_an_ordinary_string_literal_is_not_read() {
    // Assembled, because this file is inside the corpus the live sweep reads.
    let quote = '"';
    let ordinary =
        format!("    let fixture = {quote}let out = Command::new(\\{quote}git\\{quote}){quote};");
    assert!(
        !constructs_git(&ordinary),
        "an ordinary literal escapes the quotes, so it carries a different text and drops out on its own"
    );
    // The control: the same line without the literal around it is read, so the assertion above is about the
    // escaping rather than about a reader that reports nothing.
    assert!(constructs_git(&format!(
        "    let out = Command::new({quote}git{quote})"
    )));
}

/// A construction inside a **raw** string is not read either, which reading tokens closed.
///
/// It was an over-report while this file read lines: a raw string carries the spelling verbatim, so a
/// fixture written that way was reported as constructing a `git` — measured, by this file reporting itself.
/// A literal is one token, so asking a lexer answers both literal forms the same way and the over-report is
/// gone rather than declared. What remains is the one stop, in both forms, and the bound says so.
#[test]
fn a_construction_inside_a_raw_string_is_not_read() {
    // Assembled, because this file is inside the corpus the live sweep reads.
    let quote = '"';
    let raw = format!(
        "fn f() {{\n    let fixture = r#{quote}let out = Command::new({quote}git{quote}){quote}#;\n}}"
    );
    assert!(
        !constructs_git(&raw),
        "a raw string is one literal token, so what it carries is that token's text and not a call"
    );
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
