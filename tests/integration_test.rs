use assert_cmd::Command;

use std::fs;

#[test]
fn compress_and_decompress5mb() {
    let mut cmd = Command::cargo_bin("compressor").unwrap();
    cmd.arg("c")
        .arg("test.comp")
        .arg("test.txt")
        .assert()
        .success();

    let mut decomp = Command::cargo_bin("compressor").unwrap();
    decomp
        .arg("d")
        .arg("test.comp")
        .arg("test2.txt")
        .assert()
        .success();

    assert!(files_are_identical("test.txt", "test2.txt"))
}

fn files_are_identical(file_a: &str, file_b: &str) -> bool {
    let file_a_bytes = fs::read(file_a).unwrap();
    let file_b_bytes = fs::read(file_b).unwrap();
    vecs_u8_are_identical(file_a_bytes, file_b_bytes)
}

fn vecs_u8_are_identical<T: std::cmp::PartialEq>(vec_a: Vec<T>, vec_b: Vec<T>) -> bool {
    (vec_a.len() == vec_b.len()) && vec_a.iter().zip(&vec_b).all(|(a, b)| a == b)
}
