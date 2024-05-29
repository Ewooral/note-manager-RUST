use std::fs;
use std::path::PathBuf;
use assert_cmd::Command;

#[test]
fn test_create_note() {
    let title = "Test Note";
    let content = "This is a test note.";
    let mut cmd = Command::cargo_bin("note-manager-RUST").unwrap();
    cmd.args(&["create", title, content])
        .assert()
        .success()
        .stdout(format!("Created note: {}\n", title));

    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let note_path = project_dir.join(format!("notes/{}.txt", title));
    assert!(note_path.exists());
    let saved_content = fs::read_to_string(note_path).unwrap();
    assert_eq!(saved_content, content);
}

#[test]
fn test_read_note() {
    let title = "Test Note";
    let content = "This is a test note.";
    let note_path = PathBuf::from(format!("notes/{}.txt", title));
    fs::write(&note_path, content).unwrap();

    let mut cmd = Command::cargo_bin("note-manager-RUST").unwrap();
    cmd.args(&["read", title])
        .assert()
        .success()
        .stdout(format!("{}\n", content));
}

#[test]
fn test_update_note() {
    let title = "Test Note";
    let initial_content = "This is a test note.";
    let updated_content = "This is an updated test note.";
    let note_path = PathBuf::from(format!("notes/{}.txt", title));
    fs::write(&note_path, initial_content).unwrap();

    let mut cmd = Command::cargo_bin("note-manager-RUST").unwrap();
    cmd.args(&["update", title, updated_content])
        .assert()
        .success()
        .stdout(format!("Updated note: {}\n", title));

    let saved_content = fs::read_to_string(note_path).unwrap();
    assert_eq!(saved_content, updated_content);
}

#[test]
fn test_delete_note() {
    let title = "Test Note";
    let content = "This is a test note.";
    let note_path = PathBuf::from(format!("notes/{}.txt", title));
    fs::write(&note_path, content).unwrap();

    let mut cmd = Command::cargo_bin("note-manager-RUST").unwrap();
    cmd.args(&["delete", title])
        .assert()
        .success()
        .stdout(format!("Deleted note: {}\n", title));

    assert!(!note_path.exists());
}
