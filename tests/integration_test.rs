use hack_assembler::assemble;

// helper to load input and output files
fn check(asm: &str, expected: &str) {
    assert_eq!(expected.trim(), assemble(asm).unwrap().trim());
}

#[test]
fn assembles_add() {
    check(include_str!("fixtures/Add.asm"), include_str!("fixtures/Add.hack"));
}

#[test]
fn assembles_max() {
    check(include_str!("fixtures/Max.asm"), include_str!("fixtures/Max.hack"));
}

#[test]
fn assembles_maxl() {
    check(include_str!("fixtures/MaxL.asm"), include_str!("fixtures/MaxL.hack"));
}

#[test]
fn assembles_pong() {
    check(include_str!("fixtures/Pong.asm"), include_str!("fixtures/Pong.hack"));
}

#[test]
fn assembles_pongl() {
    check(include_str!("fixtures/PongL.asm"), include_str!("fixtures/PongL.hack"));
}

#[test]
fn assembles_rect() {
    check(include_str!("fixtures/Rect.asm"), include_str!("fixtures/Rect.hack"));
}

#[test]
fn assembles_rectl() {
    check(include_str!("fixtures/RectL.asm"), include_str!("fixtures/RectL.hack"));
}
