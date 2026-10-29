use std::process::{Command, Output};

fn run(binary: &str, arguments: &[&str]) -> Output {
    Command::new(binary).args(arguments).output().unwrap()
}

#[test]
fn direct_and_compact_launchers_share_root_owned_presentation() {
    let primary = env!("CARGO_BIN_EXE_liknon");
    let alias = env!("CARGO_BIN_EXE_likx");

    let plain = run(primary, &["--help"]);
    assert!(plain.status.success());
    assert!(!plain.stdout.contains(&0x1b));

    let plain_text = String::from_utf8(plain.stdout.clone()).unwrap();
    for palette in [
        "standard",
        "high-contrast",
        "protanopia",
        "deuteranopia",
        "tritanopia",
        "achromatopsia",
    ] {
        let colored = run(primary, &[&format!("--color={palette}"), "--help"]);
        assert!(colored.status.success(), "{palette}");
        assert!(colored.stdout.contains(&0x1b), "{palette}");
        assert_eq!(
            console::strip_ansi_codes(&String::from_utf8(colored.stdout).unwrap()),
            plain_text,
            "{palette}"
        );
    }

    let colored = run(primary, &["--color=standard", "--help"]);

    let alias_colored = run(alias, &["--color=standard", "--help"]);
    assert!(alias_colored.status.success());
    assert_eq!(alias_colored.stdout, colored.stdout);

    let delegated = run(primary, &["validate", "--color=standard"]);
    assert_eq!(delegated.status.code(), Some(2));
    assert!(!delegated.stderr.contains(&0x1b));
    assert!(String::from_utf8_lossy(&delegated.stderr).contains("unexpected argument '--color'"));

    let malformed = run(primary, &["--color=unknown", "--help"]);
    assert_eq!(malformed.status.code(), Some(2));
    assert!(!malformed.stderr.contains(&0x1b));
}
