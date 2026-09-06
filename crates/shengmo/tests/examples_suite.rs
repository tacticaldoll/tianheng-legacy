//! Dogfood gate: every example's product reaction behaves as its documentation declares.
//!
//! An example is a claim about what an adopter gets, and the claim is the **exit code**: a demo that reacts
//! exits 1, a run-mode that only reports events exits 0. Checking that an example merely builds says nothing
//! about either — the reaction it demonstrates could be gone entirely.
//!
//! Every example is built against **local source**, and that the patch took effect is asserted rather than
//! assumed: a `patch.crates-io` that silently fails to apply leaves the example exercising the *published*
//! crates, so the suite would pass while demonstrating nothing about this working tree. That direction is the
//! whole reason the patch is checked instead of trusted.
//!
//! It is gated behind `TIANHENG_EXAMPLES` and named on its own line in the Definition of Done and in CI,
//! because it builds seven separate crate graphs.

use std::path::{Path, PathBuf};
use std::process::Command;

/// What each example declares: the family crates it patches, the binary it runs, and the code that run owes.
struct Example {
    name: &'static str,
    family: &'static [&'static str],
    binary: &'static str,
    args: &'static [&'static str],
    /// `1` where the example exists to react; `0` where its point is that it does not.
    expected: i32,
}

const EXAMPLES: [Example; 7] = [
    Example {
        name: "guibiao-standalone",
        family: &["guibiao", "xuanji", "xingbiao"],
        binary: "demo",
        args: &[],
        expected: 1,
    },
    Example {
        name: "hunyi-standalone",
        family: &["hunyi", "xuanji", "xingbiao"],
        binary: "demo",
        args: &[],
        expected: 1,
    },
    Example {
        name: "unsafe-confinement",
        family: &["hunyi", "xuanji", "xingbiao"],
        binary: "demo",
        args: &[],
        expected: 1,
    },
    Example {
        name: "capability-catalog",
        family: &[
            "xuanji", "xingbiao", "guibiao", "hunyi", "louke", "tianheng",
        ],
        binary: "check",
        args: &["check", "--manifest-path", "Cargo.toml", "--format", "json"],
        expected: 1,
    },
    Example {
        name: "composed",
        family: &[
            "xuanji", "xingbiao", "guibiao", "hunyi", "louke", "tianheng",
        ],
        binary: "runtime_demo",
        args: &[],
        expected: 0,
    },
    Example {
        name: "sans-io-pure",
        family: &[
            "xuanji", "xingbiao", "guibiao", "hunyi", "louke", "tianheng",
        ],
        binary: "check",
        args: &["check", "--manifest-path", "Cargo.toml"],
        expected: 1,
    },
    Example {
        name: "observer-participant",
        family: &[
            "xuanji", "xingbiao", "guibiao", "hunyi", "louke", "tianheng",
        ],
        binary: "demo",
        args: &[],
        expected: 1,
    },
];

/// One isolated quality gate an example must pass.
///
/// **The two properties beside `head` were decided by comparing `label` against a literal**, twelve and
/// sixteen lines from where the label was written, in one expression. So `label` was at once the sentence
/// an operator reads and the dispatch key for whether warnings fail the build — and renaming it in the
/// table, which reads as a wording change, silently dropped `-D warnings`: clippy would still run, still
/// exit `0`, and the gate would go green having stopped reacting. That is the shape the suite's own module
/// doc names, one level up: *checking that an example merely builds says nothing about either — the
/// reaction it demonstrates could be gone entirely.*
///
/// Declared, the label decides nothing, so a rename is a rename.
///
/// Negative run, against the tuple form with `"clippy"` renamed to `"lint"` in the table and the dispatch
/// left comparing against `"clippy"` — which is what that edit does:
///
/// ```text
/// test every_example_passes_its_isolated_quality_gates ... ok
/// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
/// ```
///
/// Green, with `-D warnings` gone from every example's clippy run. Nothing else in the tree references
/// these labels, so nothing would have said so.
struct Gate {
    /// Names the gate in the assertion message, and nothing else.
    label: &'static str,
    head: &'static [&'static str],
    /// Whether warnings must fail the build. `-D warnings` is the whole point of the clippy gate.
    denies_warnings: bool,
    /// Whether the example's family patch arguments apply. `fmt` reads the source and resolves nothing.
    takes_the_family_patch: bool,
}

const GATES: [Gate; 3] = [
    Gate {
        label: "fmt",
        head: &["fmt", "--all", "--check"],
        denies_warnings: false,
        takes_the_family_patch: false,
    },
    Gate {
        label: "clippy",
        head: &["clippy", "--all-targets"],
        denies_warnings: true,
        takes_the_family_patch: true,
    },
    Gate {
        label: "doc",
        head: &["doc", "--no-deps"],
        denies_warnings: false,
        takes_the_family_patch: true,
    },
];

/// A `git` that answers about **this** repository, and about no configuration outside it.
///
/// The third of the three properties `kanhe::hermetic_git::tracked_records` owns, transcribed here with the
/// other two because `shengmo` cannot reach that owner: `kanhe` depends on `shengmo`, so the edge would
/// close a cycle. The first two — `-z`, and a strict decode — were transcribed when this enumeration was
/// converged and **this one was not**, which is what a boundary-forced copy fails at: it inherits nothing,
/// so it holds whatever was carried across by hand.
///
/// `GIT_DIR`, `GIT_WORK_TREE` and `GIT_INDEX_FILE` take precedence over discovery from `current_dir`, so a
/// set variable moves which repository is enumerated and the corpus below is a different tree's. The
/// `GIT_CONFIG_*` set closes the configuration channels in the same order the owner does; `GIT_CONFIG_COUNT`
/// is pinned to `1` with index `0` taken, so an ambient key at any index is unreachable.
fn hermetic_git() -> Command {
    let mut command = Command::new("git");
    command
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_COUNT", "1")
        .env("GIT_CONFIG_KEY_0", "core.excludesFile")
        .env("GIT_CONFIG_VALUE_0", "/dev/null")
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_CONFIG_PARAMETERS")
        .env_remove("GIT_CONFIG");
    command
}

fn workspace_root() -> Option<PathBuf> {
    shengmo::workspace::locate(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        |root| root.join("examples").is_dir(),
        shengmo::workspace::marker_set(),
    )
    // Canonicalised, because this gate COMPARES paths: `cargo metadata` prints a resolved manifest
    // path, and the manifest directory's grandparent is the same directory written differently. Measured —
    // without it every example read as unpatched. The one place a caller's answer differs from the shared
    // locator's, so it stays here rather than becoming an option nobody else would pass.
    .map(|root| std::fs::canonicalize(&root).unwrap_or(root))
}

/// `--config` arguments patching every family crate this example names to local source.
fn patch_args(root: &Path, family: &[&str]) -> Vec<String> {
    family
        .iter()
        .map(|crate_name| {
            format!(
                "--config=patch.crates-io.{crate_name}.path=\"{}\"",
                root.join("crates").join(crate_name).display()
            )
        })
        .collect()
}

fn cargo(dir: &Path, args: &[String]) -> (Option<i32>, String) {
    let out = Command::new("cargo")
        .args(args)
        .current_dir(dir)
        .env("CARGO_TERM_COLOR", "never")
        .output()
        .unwrap_or_else(|err| panic!("cannot run cargo {args:?}: {err}"));
    (
        out.status.code(),
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        ),
    )
}

fn argv(head: &[&str], patch: &[String], tail: &[&str]) -> Vec<String> {
    let mut args: Vec<String> = head.iter().map(|s| s.to_string()).collect();
    args.extend(patch.iter().cloned());
    args.extend(tail.iter().map(|s| s.to_string()));
    args
}

/// Every family crate this example names must resolve to **local source**, not to a registry version.
///
/// A `patch.crates-io` that fails to apply is silent: cargo warns and resolves the published crate, and the
/// example then demonstrates a release rather than this tree. `cargo metadata` is asked which it got.
fn assert_patched(root: &Path, dir: &Path, name: &str, family: &[&str], patch: &[String]) {
    let (code, output) = cargo(
        dir,
        &argv(&["metadata", "--format-version", "1"], patch, &[]),
    );
    assert_eq!(code, Some(0), "{name}: cargo metadata failed:\n{output}");
    for crate_name in family {
        // The decisive evidence, not a heuristic over field order: the resolved package's manifest is THIS
        // tree's. A window scan around the name field reads whichever fields cargo happened to emit next.
        let local = root
            .join("crates")
            .join(crate_name)
            .join("Cargo.toml")
            .display()
            .to_string();
        assert!(
            output.contains(&format!("\"manifest_path\":\"{local}\"")),
            "{name}: {crate_name} did not resolve to local source — patch.crates-io was silently unused, so \
             this example would demonstrate the published crate rather than this working tree"
        );
    }
}

#[test]
fn every_example_reacts_as_declared() {
    let Some(root) = workspace_root() else {
        return;
    };
    if std::env::var_os("TIANHENG_EXAMPLES").is_none() {
        eprintln!(
            "examples: skipped — set TIANHENG_EXAMPLES=1 to run it. It is named on its own line in the \
             Definition of Done and in CI, so skipping here is a cost decision rather than a hole."
        );
        return;
    }

    for example in &EXAMPLES {
        let dir = root.join("examples").join(example.name);
        assert!(
            dir.join("Cargo.toml").is_file(),
            "examples/{} carries no manifest, so the declaration above names something absent",
            example.name
        );
        let patch = patch_args(&root, example.family);
        assert_patched(&root, &dir, example.name, example.family, &patch);

        let (code, output) = cargo(&dir, &argv(&["test"], &patch, &[]));
        assert_eq!(
            code,
            Some(0),
            "{}: its own tests fail:\n{output}",
            example.name
        );

        let (code, output) = cargo(
            &dir,
            &argv(&["run", "--quiet", "--bin", example.binary], &patch, &{
                let mut tail = vec!["--"];
                tail.extend(example.args.iter().copied());
                tail
            }),
        );
        assert_eq!(
            code,
            Some(example.expected),
            "{}: `{}` exited {code:?} where its documentation declares {} — an example that stops reacting \
             is a claim about what an adopter gets that has quietly stopped being true:\n{output}",
            example.name,
            example.binary,
            example.expected
        );
    }
    eprintln!("examples ok ({} reacted as declared)", EXAMPLES.len());
}

/// Each example passes fmt, Clippy and rustdoc **in isolation**, which the workspace passes cannot see:
/// `examples/` is excluded from the workspace precisely so an adopter's build is what is tested.
#[test]
fn every_example_passes_its_isolated_quality_gates() {
    let Some(root) = workspace_root() else {
        return;
    };
    if std::env::var_os("TIANHENG_EXAMPLES").is_none() {
        // Said out loud, like its sibling above. A direction that returns green without running is
        // indistinguishable from one that ran, and the reader who most needs to know is the one who set no
        // variable and read `ok` — a local run, where the skip is the intended cost.
        eprintln!(
            "examples (isolated quality gates): skipped — set TIANHENG_EXAMPLES=1 to run it. It is named \
             on its own line in the Definition of Done and in CI, so skipping here is a cost decision \
             rather than a hole."
        );
        return;
    }
    for example in &EXAMPLES {
        let dir = root.join("examples").join(example.name);
        let patch = patch_args(&root, example.family);
        for gate in &GATES {
            let tail: &[&str] = if gate.denies_warnings {
                &["--", "-D", "warnings"]
            } else {
                &[]
            };
            let none: [String; 0] = [];
            let args = if gate.takes_the_family_patch {
                argv(gate.head, &patch, tail)
            } else {
                argv(gate.head, &none, tail)
            };
            let (code, output) = cargo(&dir, &args);
            assert_eq!(
                code,
                Some(0),
                "{}: isolated {} fails:\n{output}",
                example.name,
                gate.label
            );
        }
    }
}

/// The declared example set equals the tracked example directories, in both directions.
///
/// A directory present under `examples/` and absent from [`EXAMPLES`] is exercised by **neither** of this
/// suite's directions nor by the workflow job that runs them. That is a false negative in the gate that runs
/// the product against itself — the one gate whose silence is least likely to be questioned, because a green
/// dogfood reads as the strongest evidence there is.
///
/// The reverse matters too and is not symmetry for its own sake: an entry naming a directory the tree no longer
/// carries reads as coverage while defending nothing, which is the shape this repository's register refuses for
/// citations.
///
/// Enumerated from **tracked** content, so an untracked scratch directory is neither a failure nor an example
/// — the rule every sibling direction here follows.
///
/// This is the same guard `every_gate_running_wrapper_is_named` already applies to the wrapper constant. That
/// one direction existing while three sibling constants had none is what made this a class rather than an
/// oversight.
#[test]
fn every_tracked_example_is_declared_and_every_declaration_exists() {
    let Some(root) = workspace_root() else {
        return;
    };
    // `-z`, a strict decode, and a `git` that answers about this repository only — the three properties
    // `kanhe::hermetic_git::tracked_records` owns, spelled here because `shengmo` cannot reach `kanhe`
    // without closing a dependency cycle. The third was missing when the first two were transcribed, which
    // is the failure mode of a copy that inherits nothing: git quotes a path it cannot write plainly, a
    // replaced byte names a path the repository does not hold, and a bare `git` enumerates whichever
    // repository `GIT_DIR` names.
    let out = hermetic_git()
        .args(["ls-files", "-z", "examples"])
        .current_dir(&root)
        .output()
        .expect("run git ls-files examples");
    assert!(
        out.status.success(),
        "`git ls-files examples` failed, and a failed enumeration is not a repository with no examples"
    );
    let listing = String::from_utf8(out.stdout)
        .expect("a tracked path this reader cannot represent is refused, not renamed");
    let tracked: std::collections::BTreeSet<String> = listing
        .split('\0')
        .filter(|path| !path.is_empty())
        .filter(|path| path.ends_with("/Cargo.toml"))
        .filter_map(|path| path.strip_prefix("examples/"))
        .filter_map(|rest| rest.split_once('/'))
        .map(|(directory, _)| directory.to_string())
        .collect();
    assert!(
        !tracked.is_empty(),
        "no tracked example was enumerated, so this direction would hold over nothing"
    );

    let declared: std::collections::BTreeSet<String> =
        EXAMPLES.iter().map(|e| e.name.to_string()).collect();

    let undeclared: Vec<&String> = tracked.difference(&declared).collect();
    assert!(
        undeclared.is_empty(),
        "these tracked examples are declared by nothing, so neither direction of this suite nor the workflow \
         job that runs it exercises them: {undeclared:?}"
    );
    let absent: Vec<&String> = declared.difference(&tracked).collect();
    assert!(
        absent.is_empty(),
        "these declarations name no tracked example directory, so they read as coverage while defending \
         nothing: {absent:?}"
    );
}

/// One reading of one observation, under this builder and under a bare `Command`, with each exit status.
struct ChannelReading {
    builder_status: i32,
    builder: String,
    builder_stderr: String,
    bare_status: i32,
    bare: String,
    bare_stderr: String,
}

/// The probe half of the behavioural case below, reached as a child process.
///
/// Reports one observation twice — under this builder and under a bare `Command` — with **each reading's
/// exit status beside it**. Folded into stdout, a `git` that failed produced an empty reading, and the
/// worktree case's isolated value *is* the empty string: a failure passed as isolation.
#[test]
fn hermetic_channel_probe() {
    let Some(judged) = std::env::var_os("SHENGMO_PROBE_JUDGED") else {
        return;
    };
    let read = std::env::var("SHENGMO_PROBE_READ").expect("the parent names the observation");
    let judged = std::path::Path::new(&judged);
    let arguments: Vec<&str> = match read.as_str() {
        "log" => vec!["log", "-1", "--format=%s"],
        "ls-files" => vec!["ls-files"],
        "status" => vec!["status", "--porcelain"],
        "config-probe-marker" => vec!["config", "--default", "isolated", "--get", "probe.marker"],
        other => panic!("the parent named an observation this probe does not make: {other}"),
    };
    let subject = |mut command: Command| {
        let out = command
            .args(&arguments)
            .current_dir(judged)
            .output()
            .expect("run git");
        (
            out.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&out.stdout).trim().to_string(),
            String::from_utf8_lossy(&out.stderr).trim().to_string(),
        )
    };
    let (builder_code, builder, builder_stderr) = subject(hermetic_git());
    let (bare_code, bare, bare_stderr) = subject(Command::new("git"));
    println!(
        "PROBE_BEGIN\nbuilder-status={builder_code}\nbuilder={builder}\nbuilder-stderr={builder_stderr}\nbare-status={bare_code}\nbare={bare}\nbare-stderr={bare_stderr}\nPROBE_END"
    );
}

/// No ambient channel moves what this builder reads — **asked of a run, one channel at a time**.
///
/// This copy holds the owner's isolation by transcription, because `shengmo` cannot reach `kanhe` without
/// closing a dependency cycle. **The cases are not transcribed with it.** Written out per site, the matrix
/// covered three selectors and one configuration channel here and the same four in the sibling, while
/// `GIT_CONFIG`, the two file channels and the indexed channel were in none of them — a matrix per site is a
/// matrix that diverges per site. `crates/kanhe/tests/fixtures/hermetic_channels.tsv` holds them once and
/// every builder consumes it, so a channel added there is a case this copy starts owing.
#[test]
fn no_ambient_channel_moves_what_the_examples_suite_builder_reads() {
    let Some(root_of) = workspace_root() else {
        return;
    };
    let inventory =
        std::fs::read_to_string(root_of.join("crates/kanhe/tests/fixtures/hermetic_channels.tsv"))
            .expect("the channel inventory is readable");

    let root = std::env::temp_dir().join(format!("examples-suite-channels-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    // `xingbiao::claim_scratch` owns this elsewhere and `shengmo` cannot reach it without a dependency
    // edge, so the property it holds — a scratch root that refuses to adopt a pre-existing path — is held
    // here instead of dropped.
    assert!(
        std::fs::symlink_metadata(&root).is_err(),
        "the scratch root must not exist before it is made"
    );
    std::fs::create_dir_all(&root).expect("create the fixture root");
    let build = |name: &str| {
        let dir = root.join(name);
        std::fs::create_dir_all(&dir).expect("create the fixture repository");
        for args in [
            &["init", "-q", "."][..],
            &["config", "user.email", "fixture@example.invalid"][..],
            &["config", "user.name", name][..],
        ] {
            assert!(
                hermetic_git()
                    .args(args)
                    .current_dir(&dir)
                    .status()
                    .expect("run git")
                    .success(),
                "the fixture repository is built"
            );
        }
        std::fs::write(dir.join(format!("{name}.txt")), name).expect("write the fixture file");
        for args in [&["add", "-A"][..], &["commit", "-qm", name][..]] {
            assert!(
                hermetic_git()
                    .args(args)
                    .current_dir(&dir)
                    .status()
                    .expect("run git")
                    .success(),
                "the fixture commit is made"
            );
        }
        dir
    };
    let judged = build("judged");
    let decoy = build("decoy");
    let config = root.join("ambient.gitconfig");
    std::fs::write(&config, "[probe]\n\tmarker = ambient-probe\n")
        .expect("write the ambient config");

    let cases: Vec<Vec<String>> = inventory
        .lines()
        .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
        .map(|line| line.split('\t').map(str::to_string).collect())
        .collect();
    assert!(
        cases.len() >= 8,
        "the channel inventory collapsed to {} case(s); a matrix that shrinks is one this check stops \
         asking about",
        cases.len()
    );

    let mut readings = Vec::new();
    for case in &cases {
        let channel = &case[0];
        let injection = &case[1];
        let read = &case[2];
        // `(empty)` rather than a blank field: a trailing tab is whitespace this repository refuses.
        let isolated = match case.get(3).map(String::as_str) {
            Some("(empty)") | None => String::new(),
            Some(value) => value.to_string(),
        };
        let mut probe = Command::new(std::env::current_exe().expect("this test binary"));
        probe.args([
            "--exact",
            "hermetic_channel_probe",
            "--nocapture",
            "--test-threads=1",
        ]);
        match injection.as_str() {
            "git-dir" => {
                probe.env(channel, decoy.join(".git"));
            }
            "work-tree" => {
                probe.env(channel, &decoy);
            }
            "index" => {
                probe.env(channel, decoy.join(".git/index"));
            }
            "config-file" => {
                probe.env(channel, &config);
            }
            other => {
                if let Some(value) = other.strip_prefix("literal:") {
                    probe.env(channel, value);
                } else if let Some(pair) = other.strip_prefix("indexed:") {
                    let (key, value) = pair.split_once('=').unwrap_or_else(|| {
                        panic!("an indexed injection is `key=value`, got {pair:?}")
                    });
                    probe
                        .env(channel, "1")
                        .env("GIT_CONFIG_KEY_0", key)
                        .env("GIT_CONFIG_VALUE_0", value);
                } else {
                    panic!("the inventory names an injection this runner does not make: {other}");
                }
            }
        }
        let out = probe
            .env("SHENGMO_PROBE_JUDGED", &judged)
            .env("SHENGMO_PROBE_READ", read)
            .output()
            .expect("run the probe child");
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let reported = stdout
            .split_once("PROBE_BEGIN\n")
            .and_then(|(_, rest)| rest.split_once("PROBE_END"))
            .map(|(body, _)| body.to_string())
            .unwrap_or_else(|| {
                panic!("the probe child produced no reading for {channel}:\n{stdout}")
            });
        let line = |key: &str| {
            reported
                .lines()
                .find_map(|line| line.strip_prefix(key))
                .unwrap_or_else(|| {
                    panic!("the probe reported no `{key}` line for {channel}:\n{reported}")
                })
                .to_string()
        };
        let code = |key: &str| {
            line(key)
                .parse::<i32>()
                .expect("the probe reports a status")
        };
        readings.push((
            channel.clone(),
            ChannelReading {
                builder_status: code("builder-status="),
                builder: line("builder="),
                builder_stderr: line("builder-stderr="),
                bare_status: code("bare-status="),
                bare: line("bare="),
                bare_stderr: line("bare-stderr="),
            },
            isolated,
        ));
    }

    let _ = std::fs::remove_dir_all(&root);

    for (channel, reading, isolated) in readings {
        for (who, code, stderr) in [
            ("builder", reading.builder_status, &reading.builder_stderr),
            ("bare", reading.bare_status, &reading.bare_stderr),
        ] {
            assert_eq!(
                code, 0,
                "the {who} `git` under {channel} exited {code}, so its reading is a failure rather than an \
                 answer: {stderr}"
            );
        }
        // The control first: this channel does move this reading, so the assertion after it is a difference
        // rather than an environment that never arrived.
        assert_ne!(
            reading.bare, isolated,
            "a bare `Command` read the same under {channel} as without it, so this case demonstrates no \
             channel and the assertion below would hold for the wrong reason"
        );
        assert_eq!(
            reading.builder, isolated,
            "this builder followed {channel}, so a verdict behind it is about a tree or a configuration \
             nobody asked for"
        );
    }
}
