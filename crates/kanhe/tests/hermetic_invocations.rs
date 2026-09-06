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

/// One environment operation: the method, the variable it names, and the value it assigns.
///
/// **A name is not an operation, and this modelled one as the other.** Collected as bare variable names,
/// the set could not tell `.env_remove("GIT_DIR")` from `.env("GIT_DIR", "/tmp/other")` — a copy that
/// *points* the selector somewhere satisfied a requirement that it *clear* it — nor `GIT_CONFIG_COUNT`
/// pinned to `"1"` from the same variable set to `"0"`, which reopens the ambient-key channel the builder's
/// own header spends a paragraph closing. Both were live falsifiers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Operation {
    method: String,
    variable: String,
    /// The assigned value for `env`, and `None` for `env_remove`, which assigns nothing.
    value: Option<String>,
}

/// The environment operations a site declaring [`Isolation::Isolated`] must make, written out here and held
/// against the builder **both ways**.
///
/// Compared one way it caught the builder dropping an operation and never the builder gaining one, so a
/// twelfth would have been required of no copy. Written as bare variable names it could not tell
/// `.env_remove("GIT_DIR")` from `.env("GIT_DIR", "/tmp/other")`, nor `GIT_CONFIG_COUNT` pinned to `"1"`
/// from the same variable set to `"0"` — a copy that reopens the ambient-key channel while satisfying it.
fn expected_operations() -> BTreeSet<Operation> {
    let assigns = |variable: &str, value: &str| Operation {
        method: "env".to_string(),
        variable: variable.to_string(),
        value: Some(value.to_string()),
    };
    let clears = |variable: &str| Operation {
        method: "env_remove".to_string(),
        variable: variable.to_string(),
        value: None,
    };
    [
        assigns("GIT_CONFIG_GLOBAL", "/dev/null"),
        assigns("GIT_CONFIG_SYSTEM", "/dev/null"),
        assigns("GIT_CONFIG_NOSYSTEM", "1"),
        assigns("GIT_CONFIG_COUNT", "1"),
        assigns("GIT_CONFIG_KEY_0", "core.excludesFile"),
        assigns("GIT_CONFIG_VALUE_0", "/dev/null"),
        clears("GIT_DIR"),
        clears("GIT_WORK_TREE"),
        clears("GIT_INDEX_FILE"),
        clears("GIT_CONFIG_PARAMETERS"),
        clears("GIT_CONFIG"),
    ]
    .into_iter()
    .collect()
}

/// What this reader could decide about a file.
///
/// **Three states, because a file it cannot parse is not a file that constructs nothing.** The tokeniser's
/// failure arm fell back to an exact substring, which answers `false` for a construction split across lines
/// — so an unparseable file carrying one was reported clean, silently, which is the one direction the Core
/// Contract forbids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reading {
    Constructs,
    DoesNot,
    Undecidable,
}

/// A string literal's **value**, not its rendering.
///
/// `Literal::to_string` gives back the source spelling, so `r"git"` and `"\x67it"` — both of which decode to
/// `git` — compared unequal to `"git"`, and a construction written either way was not read.
fn literal_value(literal: &syn::Lit) -> Option<String> {
    match literal {
        syn::Lit::Str(text) => Some(text.value()),
        _ => None,
    }
}

/// That expression, where it is a string literal.
fn string_of(expression: &syn::Expr) -> Option<String> {
    match expression {
        syn::Expr::Lit(literal) => literal_value(&literal.lit),
        _ => None,
    }
}

/// **Read with a parser, not by counting tokens around a name.**
///
/// Four rounds of findings in this file were one cause, and it survived a repair: reading *lines* gave way
/// to reading *tokens*, and the readers stayed positional — `trees[index - 3]` for the owner segment, a
/// literal compared by its rendering, a value at `inner.get(2)` or dropped. Each round closed the instance
/// the review brought. `refusal_register`'s own header records this repository already learning it once:
/// a reader that is *text over Rust* is not exhaustive over the language, and reading its own Rust with a
/// real parser is what closed that floor. `syn` is already a dev-dependency here for exactly that.
///
/// A call's callee, its argument count and each argument's decoded value are what a parse gives; none of
/// them is an offset from something else.
#[derive(Default)]
struct Constructions {
    found: bool,
    /// Names this file binds to `Command` — `use std::process::Command as Cmd` makes `Cmd` one.
    aliases: BTreeSet<String>,
}

impl Constructions {
    /// Every name this file may spell `Command` as, collected before the calls are read.
    ///
    /// **A rename is decidable inside one file and not outside it.** `use std::process::Command as Cmd`
    /// makes `Cmd::new("git")` the same construction, and reading only the segment `Command` missed it. What
    /// a rename *elsewhere* binds is not written down anywhere a parse tree carries — the floor
    /// `repository-checks/a-construction-shape-the-register-s-reader-does-not-model-a-stated-bound` already
    /// names for this repository's other reader of its own Rust, reached here by the same road.
    fn bind_aliases(&mut self, file: &syn::File) {
        self.aliases.insert("Command".to_string());
        for item in &file.items {
            if let syn::Item::Use(imported) = item {
                Self::walk_use(&imported.tree, &mut self.aliases);
            }
        }
    }

    fn walk_use(tree: &syn::UseTree, aliases: &mut BTreeSet<String>) {
        match tree {
            syn::UseTree::Path(path) => Self::walk_use(&path.tree, aliases),
            syn::UseTree::Group(group) => {
                for item in &group.items {
                    Self::walk_use(item, aliases);
                }
            }
            syn::UseTree::Rename(renamed) if renamed.ident == "Command" => {
                aliases.insert(renamed.rename.to_string());
            }
            _ => {}
        }
    }
}

impl<'ast> syn::visit::Visit<'ast> for Constructions {
    /// A construction inside a macro is read, by visiting the body the macro was handed.
    ///
    /// `dbg!(Command::new("git"))` is a construction the default walk never reaches, because a macro's
    /// tokens are not expressions until something parses them. The body is parsed as an expression list
    /// where it is one; a body that is not — an `assert!` with a message, a `matches!` pattern — carries no
    /// call this reader would have read anyway, and the file is not made undecidable for it.
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        use syn::punctuated::Punctuated;
        if let Ok(arguments) =
            node.parse_body_with(Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated)
        {
            for argument in &arguments {
                syn::visit::Visit::visit_expr(self, argument);
            }
        }
        syn::visit::visit_macro(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(callee) = &*node.func {
            let segments: Vec<String> = callee
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect();
            let names_command = segments.len() >= 2
                && self.aliases.contains(&segments[segments.len() - 2])
                && segments[segments.len() - 1] == "new";
            if names_command
                && node.args.len() == 1
                && string_of(&node.args[0]).as_deref() == Some("git")
            {
                self.found = true;
            }
        }
        syn::visit::visit_expr_call(self, node);
    }
}

/// Whether `text` constructs a `git`, or says it could not decide.
fn constructs_git(text: &str) -> Reading {
    let Ok(parsed) = syn::parse_file(text) else {
        return Reading::Undecidable;
    };
    let mut constructions = Constructions::default();
    constructions.bind_aliases(&parsed);
    syn::visit::Visit::visit_file(&mut constructions, &parsed);
    if constructions.found {
        Reading::Constructs
    } else {
        Reading::DoesNot
    }
}

/// The two readings a direction asserts, so a tri-state reader is not flattened at every call site.
fn reads(text: &str) -> bool {
    matches!(constructs_git(text), Reading::Constructs)
}

/// Every `.env(…)` / `.env_remove(…)` operation a syntax node makes, values included.
///
/// A value spelled as an identifier is kept as that identifier and resolved by the caller against the
/// builder's own constants: `hermetic` writes `.env("GIT_CONFIG_KEY_0", EXCLUDES_SETTING)` where a copy that
/// cannot reach it writes the string, and those are the same operation.
#[derive(Default)]
struct Environment {
    made: BTreeSet<Operation>,
    /// Identifiers passed to `env_remove` — a loop's element rather than a variable's name.
    bound: BTreeSet<String>,
}

impl<'ast> syn::visit::Visit<'ast> for Environment {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let method = node.method.to_string();
        if method == "env" || method == "env_remove" {
            // A removal whose argument is a binding rather than a literal names the loop element, which
            // `IteratedRemoval` is what reads.
            if method == "env_remove" {
                if let Some(syn::Expr::Path(bound)) = node.args.first() {
                    if let Some(ident) = bound.path.get_ident() {
                        self.bound.insert(ident.to_string());
                    }
                }
            }
            if let Some(variable) = node.args.first().and_then(string_of) {
                let value = node.args.iter().nth(1).and_then(|assigned| match assigned {
                    syn::Expr::Lit(literal) => literal_value(&literal.lit),
                    syn::Expr::Path(constant) => {
                        constant.path.get_ident().map(|ident| ident.to_string())
                    }
                    _ => None,
                });
                self.made.insert(Operation {
                    variable,
                    value: if method == "env_remove" { None } else { value },
                    method,
                });
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

/// Every string literal's value in an expression — the two arrays [`kanhe::hermetic_git::hermetic`] iterates.
#[derive(Default)]
struct Strings {
    found: BTreeSet<String>,
}

impl<'ast> syn::visit::Visit<'ast> for Strings {
    fn visit_lit(&mut self, node: &'ast syn::Lit) {
        if let Some(value) = literal_value(node) {
            self.found.insert(value);
        }
    }
}

/// The value a `const` in `file` declares, as a string.
fn const_string(file: &syn::File, name: &str) -> Option<String> {
    const_strings(file, name).into_iter().next()
}

/// Every string a `const` in `file` declares — one for a plain value, several for an array.
///
/// An item named by the parser, so there is no terminator to pick. Sliced out of text by searching for one,
/// the two single-line `const` openings took the *next function's* closing brace — an 87-line span for a
/// one-line subject — and the arm written for them was a branch no input could reach.
fn const_strings(file: &syn::File, name: &str) -> BTreeSet<String> {
    for item in &file.items {
        if let syn::Item::Const(declared) = item {
            if declared.ident == name {
                let mut strings = Strings::default();
                syn::visit::Visit::visit_expr(&mut strings, &declared.expr);
                return strings.found;
            }
        }
    }
    BTreeSet::new()
}

/// Whether `hermetic` iterates `array` and calls `env_remove` on what it binds.
///
/// The membership of a constant is not an operation: a `const` kept while its loop is deleted is a name the
/// builder no longer acts on, and reading the array alone reported the removals as still made. The loop is
/// what performs them, so the loop is what is read — its iterable being that constant, and its body calling
/// `env_remove` on the element it binds rather than on anything else.
struct IteratedRemoval<'a> {
    array: &'a str,
    found: bool,
}

impl<'ast> syn::visit::Visit<'ast> for IteratedRemoval<'_> {
    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        let over_the_array = matches!(
            &*node.expr,
            syn::Expr::Path(path) if path.path.is_ident(self.array)
        );
        if over_the_array {
            if let syn::Pat::Ident(binding) = &*node.pat {
                let mut removals = Environment::default();
                syn::visit::Visit::visit_block(&mut removals, &node.body);
                let element = binding.ident.to_string();
                if removals.bound.contains(&element) {
                    self.found = true;
                }
            }
        }
        syn::visit::visit_expr_for_loop(self, node);
    }
}

fn iterated_into_env_remove(file: &syn::File, array: &str) -> bool {
    for item in &file.items {
        if let syn::Item::Fn(function) = item {
            if function.sig.ident == "hermetic" {
                let mut reader = IteratedRemoval {
                    array,
                    found: false,
                };
                syn::visit::Visit::visit_block(&mut reader, &function.block);
                return reader.found;
            }
        }
    }
    false
}

/// Every environment operation [`kanhe::hermetic_git::hermetic`] makes, read from the builder's own items.
///
/// `hermetic`'s own body for the calls it makes, and the two arrays it iterates for the names it removes.
/// The fixture-side `commit` names `GIT_AUTHOR_DATE` and `GIT_COMMITTER_DATE`, which are no part of what a
/// *read* inherits and are outside these items by construction rather than by a filter over a wider span.
///
/// A value the builder spells as a constant is resolved to that constant's own value, so a copy writing the
/// string and the builder writing the name are read as the one operation they are.
fn environment_operations_of(builder: &str) -> BTreeSet<Operation> {
    let parsed = syn::parse_file(builder).expect("the builder is Rust this reader can parse");

    let mut inside = Environment::default();
    let mut seen = false;
    for item in &parsed.items {
        if let syn::Item::Fn(function) = item {
            if function.sig.ident == "hermetic" {
                seen = true;
                syn::visit::Visit::visit_block(&mut inside, &function.block);
            }
        }
    }
    assert!(
        seen,
        "the builder no longer holds a `fn hermetic`, so this reader's subject is not its subject"
    );

    let mut found: BTreeSet<Operation> = inside
        .made
        .into_iter()
        .map(|operation| Operation {
            value: operation
                .value
                .map(|value| const_string(&parsed, &value).unwrap_or(value)),
            ..operation
        })
        .collect();

    // **The loop, not the constant.** Expanding an array into removals because the array still exists let
    // the constant stand in for an operation the builder no longer performs: deleting
    // `for selector in REPOSITORY_SELECTORS { command.env_remove(selector) }` while keeping the array left
    // this check green over a builder that had stopped clearing every repository selector. What is read now
    // is a `for` loop in `hermetic` whose iterable IS that constant and whose body calls `env_remove` on the
    // element it binds.
    let mut arrays = 0usize;
    for array in ["CONFIG_CHANNELS", "REPOSITORY_SELECTORS"] {
        if !iterated_into_env_remove(&parsed, array) {
            continue;
        }
        let names = const_strings(&parsed, array);
        if names.is_empty() {
            continue;
        }
        arrays += 1;
        for variable in names {
            found.insert(Operation {
                method: "env_remove".to_string(),
                variable,
                value: None,
            });
        }
    }
    assert_eq!(
        arrays, 2,
        "`hermetic` no longer iterates both arrays into `env_remove`, so this reader would derive a set \
         from whichever half it still performs"
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

#[test]
fn every_git_this_repository_constructs_is_the_builders_or_is_declared() {
    let Some(root) = workspace_root() else {
        return;
    };

    let tracked = kanhe::hermetic_git::tracked_paths(&root, &["*.rs"]).expect(
        "the tracked Rust is enumerable; a failed enumeration is not a repository with no sources",
    );
    let mut constructing = BTreeSet::new();
    let mut undecidable = Vec::new();
    let mut examined = 0usize;
    for path in &tracked {
        let text = std::fs::read_to_string(root.join(path)).unwrap_or_else(|err| {
            panic!("cannot read tracked file '{path}' — a file this check claims to have inspected must have been read: {err}")
        });
        examined += 1;
        match constructs_git(&text) {
            Reading::Constructs => {
                constructing.insert(path.clone());
            }
            Reading::DoesNot => {}
            Reading::Undecidable => undecidable.push(path.clone()),
        }
    }
    assert!(
        examined > 0,
        "no tracked Rust was inspected, so this check would report clean over nothing"
    );
    // A file this reader cannot parse is not a file that constructs nothing. Its predecessor fell back to a
    // substring, which answers `false` for a construction split across lines — clean, silently, over a file
    // it never read.
    assert!(
        undecidable.is_empty(),
        "tracked Rust this reader could not parse, so whether it constructs a `git` was never decided:\n  {}",
        undecidable.join("\n  ")
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
    let makes = environment_operations_of(&builder);
    let required = expected_operations();
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
        let parsed = syn::parse_file(&text).unwrap_or_else(|err| {
            panic!("the declared site '{path}' is not Rust this reader parses: {err}")
        });
        let mut inside = Environment::default();
        syn::visit::Visit::visit_file(&mut inside, &parsed);
        for operation in &makes {
            if !inside.made.contains(operation) {
                missing.push(format!(
                    "  {path}: makes no `.{}({:?}{})` call",
                    operation.method,
                    operation.variable,
                    operation
                        .value
                        .as_ref()
                        .map_or(String::new(), |value| format!(", {value:?}"))
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
    assert!(!reads(
        "/// `Command::new(\"git\")` is what this forbids\nfn f() {}"
    ));
    assert!(!reads(
        "fn f() {\n    // a bare Command::new(\"git\") inherits the environment\n}"
    ));
    // **Every comment form, which is what asking a lexer buys.** A trimmed-line reader saw `//` and not
    // `/* … */`, so a block comment carrying the spelling was read as a construction — the prose bound
    // saying the opposite two files away.
    assert!(!reads(
        "fn f() {\n    /* a bare Command::new(\"git\") inherits the environment */\n}"
    ));
    assert!(!reads(
        "fn f() {\n    let x = 1; /* Command::new(\"git\") */\n}"
    ));
    assert!(reads("fn f() {\n    let out = Command::new(\"git\");\n}"));
}

/// A construction rustfmt split across lines is still a construction.
///
/// The other half of what reading lines cost: a substring over one trimmed line saw neither the opening nor
/// the argument, so a file whose construction wrapped was **not** in the constructing set and the check's
/// own requirement — that every file constructing a `git` is declared — was wider than its reader.
#[test]
fn a_construction_split_across_lines_is_read() {
    assert!(reads(
        "fn f() {\n    let out = Command::new(\n        \"git\",\n    );\n}"
    ));
    // The control: the same wrapping around a different program is not read, so the assertion above is
    // about the argument rather than about a reader that reports every `new`.
    assert!(!reads(
        "fn f() {\n    let out = Command::new(\n        \"cargo\",\n    );\n}"
    ));
    assert!(!reads("fn f() {\n    let out = Other::new(\"git\");\n}"));
}

/// A `git` constructed through a program value is not read.
///
/// The stop the bound of that name declares, pinned by a direction that exercises **it**. Whether a value
/// names `git` is not decidable from the line that constructs it, and the builder itself is written that way
/// — `Command::new(program)` — which is why the stop exists rather than being closed.
#[test]
fn a_construction_through_a_program_value_is_not_read() {
    assert!(!reads(
        "fn f() {\n    let mut command = Command::new(program);\n}"
    ));
    assert!(!reads("fn f() {\n    let out = Command::new(&exe);\n}"));
    // The control: the literal form in the same file is read, so the assertions above are about the value
    // rather than about a reader that reports nothing.
    assert!(reads(
        "fn f() {\n    let mut command = Command::new(\"git\");\n}"
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
/// A literal is one expression, so what it carries is that expression's value and not a call — and a file
/// that writes Rust and compiles it is where that matters. Reading lines, this split in two by escaping;
/// read as syntax, both literal forms answer the same way and the raw-string over-report is gone.
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
    let ordinary = format!(
        "fn f() {{\n    let fixture = {quote}let out = Command::new(\\{quote}git\\{quote}){quote};\n}}"
    );
    assert!(
        !reads(&ordinary),
        "an ordinary literal escapes the quotes, so it carries a different text and drops out on its own"
    );
    // The control: the same line without the literal around it is read, so the assertion above is about the
    // escaping rather than about a reader that reports nothing.
    assert!(reads(&format!(
        "fn f() {{\n    let out = Command::new({quote}git{quote});\n}}"
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
        !reads(&raw),
        "a raw string is one literal token, so what it carries is that token's text and not a call"
    );
}

/// A construction spelled another way is the same construction.
///
/// `Literal::to_string` gives back the **source rendering**, so `r"git"` and `"\x67it"` — both of which
/// decode to `git` — compared unequal to `"git"` and neither was read. A parser decodes, so the reader
/// answers about the program named rather than about how it was typed.
#[test]
fn a_construction_spelled_another_way_is_still_read() {
    assert!(reads("fn f() {\n    let out = Command::new(r\"git\");\n}"));
    assert!(reads(
        "fn f() {\n    let out = Command::new(\"\\x67it\");\n}"
    ));
    // The control: another program spelled the same ways is not read.
    assert!(!reads(
        "fn f() {\n    let out = Command::new(r\"cargo\");\n}"
    ));
}

/// An operation is its method and its value, not the variable it names.
///
/// Collected as bare names, the set could not tell clearing a selector from **pointing** it somewhere, nor
/// a count pinned to `"1"` from the same count set to `"0"` — which reopens the ambient-key channel the
/// builder's header spends a paragraph closing. Both were falsifiers a review supplied and both passed.
#[test]
fn an_operation_is_its_method_and_its_value() {
    let operations = |source: &str| {
        let parsed = syn::parse_file(source).expect("the probe is Rust");
        let mut inside = Environment::default();
        syn::visit::Visit::visit_file(&mut inside, &parsed);
        inside.made
    };
    let clears = operations("fn f() {\n    c.env_remove(\"GIT_DIR\");\n}");
    let points = operations("fn f() {\n    c.env(\"GIT_DIR\", \"/tmp/other\");\n}");
    assert_ne!(
        clears, points,
        "clearing a repository selector and pointing it somewhere are not one operation"
    );

    let pinned = operations("fn f() {\n    c.env(\"GIT_CONFIG_COUNT\", \"1\");\n}");
    let opened = operations("fn f() {\n    c.env(\"GIT_CONFIG_COUNT\", \"0\");\n}");
    assert_ne!(
        pinned, opened,
        "a count pinned to one and a count set to zero are not one operation: the second reopens the \
         ambient-key channel while naming the same variable"
    );

    // And the builder's own constant-spelled value is resolved, so a copy writing the string and the
    // builder writing the name are read as the one operation they are.
    assert!(
        expected_operations().contains(&Operation {
            method: "env".to_string(),
            variable: "GIT_CONFIG_KEY_0".to_string(),
            value: Some("core.excludesFile".to_string()),
        }),
        "the expected set carries the resolved value, not the constant's name"
    );
}

/// A file this reader cannot parse is undecidable, not a file that constructs nothing.
///
/// The tokeniser's failure arm fell back to an exact substring, which answers `false` for a construction
/// split across lines — so an unparseable file carrying one was reported clean, silently, which is the one
/// direction the Core Contract forbids. The corpus direction names such a path and fails.
#[test]
fn a_file_this_reader_cannot_parse_is_undecidable() {
    assert_eq!(constructs_git("fn f( {"), Reading::Undecidable);
    assert_eq!(
        constructs_git("fn f() {\n    let out = Command::new(\n        \"git\",\n    );\n}"),
        Reading::Constructs
    );
    assert_eq!(constructs_git("fn f() {}"), Reading::DoesNot);
}

/// A construction through a rename or inside a macro is the same construction.
///
/// Both were escapes a review supplied and both worked: reading only the segment `Command` missed
/// `use std::process::Command as Cmd; Cmd::new("git")`, and the default walk never reaches a macro's tokens,
/// so `dbg!(Command::new("git"))` was a construction in an undeclared file that this check reported as none.
#[test]
fn a_construction_through_a_rename_or_inside_a_macro_is_read() {
    assert!(reads(
        "use std::process::Command as Cmd;\nfn f() {\n    let c = Cmd::new(\"git\");\n}"
    ));
    assert!(reads("fn f() {\n    dbg!(Command::new(\"git\"));\n}"));
    assert!(reads(
        "use std::process::Command as Cmd;\nfn f() {\n    dbg!(Cmd::new(\"git\"));\n}"
    ));
    assert!(reads(
        "fn f() {\n    let c = std::process::Command::new(\"git\");\n}"
    ));

    // The controls: a rename this file does not bind is not `Command`, and a macro carrying another program
    // is not read — so the assertions above are about the binding and the body rather than about a reader
    // that reports every `new`.
    assert!(!reads("fn f() {\n    let c = Cmd::new(\"git\");\n}"));
    assert!(!reads("fn f() {\n    dbg!(Command::new(\"cargo\"));\n}"));
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
