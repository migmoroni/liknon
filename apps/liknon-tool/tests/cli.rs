use std::process::{Command, Output};

fn run(binary: &str, arguments: &[&str]) -> Output {
    Command::new(binary).args(arguments).output().unwrap()
}

#[test]
fn direct_and_compact_tool_shells_share_the_visual_contract() {
    let primary = env!("CARGO_BIN_EXE_liknon-tool");
    let alias = env!("CARGO_BIN_EXE_likt");

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

    let low_vision = run(primary, &["--presentation=low-vision", "--help"]);
    assert!(low_vision.status.success());
    assert!(low_vision.stdout.contains(&0x1b));

    let alias_colored = run(alias, &["--color=standard", "--help"]);
    assert!(alias_colored.status.success());
    assert_eq!(alias_colored.stdout, colored.stdout);

    let malformed = run(primary, &["--color=unknown", "--help"]);
    assert_eq!(malformed.status.code(), Some(2));
    assert!(!malformed.stderr.contains(&0x1b));
}
