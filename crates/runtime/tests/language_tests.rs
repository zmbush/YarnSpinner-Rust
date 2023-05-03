//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/LanguageTests.cs>

use crate::test_base::*;
use std::collections::HashMap;
use yarn_slinger_compiler::prelude::*;
use yarn_slinger_core::prelude::*;

mod test_base;

#[test]
#[ignore]
fn test_example_script() {
    todo!("Not ported yet")
}

#[test]
fn can_compile_space_demo() -> std::io::Result<()> {
    let test_base = TestBase::default();
    let sally_path = TestBase::space_demo_scripts_path().join("Sally.yarn");
    let ship_path = TestBase::space_demo_scripts_path().join("Ship.yarn");

    let compilation_job_sally = CompilationJob::default()
        .read_file(&sally_path)?
        .with_library(test_base.dialogue.library.clone());
    let compilation_job_sally_and_ship = CompilationJob::default()
        .read_file(&sally_path)?
        .read_file(&ship_path)?
        .with_library(test_base.dialogue.library.clone());

    let _result_sally = compile(compilation_job_sally).unwrap_pretty();
    let _result_sally_and_ship = compile(compilation_job_sally_and_ship).unwrap_pretty();

    Ok(())
}

#[test]
#[should_panic]
fn test_merging_nodes() {
    let test_base = TestBase::default();
    let sally_path = TestBase::space_demo_scripts_path().join("Sally.yarn");
    let ship_path = TestBase::space_demo_scripts_path().join("Ship.yarn");

    let compilation_job_sally = CompilationJob::default()
        .read_file(&sally_path)
        .unwrap()
        .with_library(test_base.dialogue.library.clone());
    let compilation_job_sally_and_ship = CompilationJob::default()
        .read_file(&sally_path)
        .unwrap()
        .read_file(&ship_path)
        .unwrap()
        .with_library(test_base.dialogue.library.clone());

    let result_sally = compile(compilation_job_sally).unwrap_pretty();
    let result_sally_and_ship = compile(compilation_job_sally_and_ship).unwrap_pretty();

    // Loading code with the same contents should throw
    let _combined_not_working = Program::combine(vec![
        result_sally.program.unwrap(),
        result_sally_and_ship.program.unwrap(),
    ]);
}

#[test]
#[ignore]
fn test_end_of_notes_with_options_not_added() {
    todo!("Not ported yet")
}

#[test]
fn test_node_headers() {
    let path = TestBase::test_data_path().join("Headers.yarn");
    let result = compile(CompilationJob::default().read_file(&path).unwrap()).unwrap_pretty();
    let program = result.program.as_ref().unwrap();
    assert_eq!(program.nodes.len(), 6);

    for tag in &["one", "two", "three"].map(|s| s.to_owned()) {
        assert!(program.nodes["Tags"].tags.contains(tag));
    }

    let headers: HashMap<_, _> = vec![
        ("EmptyTags", vec![("title", "EmptyTags"), ("tags", "")]),
        (
            "ArbitraryHeaderWithValue",
            vec![
                ("title", "ArbitraryHeaderWithValue"),
                ("arbitraryheader", "some-arbitrary-text"),
            ],
        ),
        ("Tags", vec![("title", "Tags"), ("tags", "one two three")]),
        ("SingleTagOnly", vec![("title", "SingleTagOnly")]),
        (
            "Comments",
            vec![("title", "Comments"), ("tags", "one two three")],
        ),
        (
            "LotsOfHeaders",
            vec![
                ("contains", "lots"),
                ("title", "LotsOfHeaders"),
                ("this", "node"),
                ("of", ""),
                ("blank", ""),
                ("others", "are"),
                ("headers", ""),
                ("some", "are"),
                ("not", ""),
            ],
        ),
    ]
    .into_iter()
    .collect();
    assert_eq!(program.nodes.len(), headers.len());
    for (node_name, expected_headers) in headers {
        let node = &program.nodes[node_name];
        assert_eq!(node.headers.len(), expected_headers.len());
        for header in &node.headers {
            let expected_header = expected_headers
                .iter()
                .find(|(k, _)| k == &header.key)
                .unwrap();
            assert_eq!(header.value, expected_header.1);
        }
    }

    let path = path.to_string_lossy().to_string();

    assert!(result.file_tags.contains_key(&path));
    assert_eq!(1, result.file_tags.len());
    assert!(result.file_tags[&path].contains(&"file_header".to_owned()));
    assert_eq!(1, result.file_tags[&path].len());
}

#[test]
#[ignore]
fn test_invalid_characters_in_node_title() {
    todo!("Not ported yet")
}

#[test]
#[ignore]
fn test_number_plurals() {
    todo!("Not ported yet")
}

#[test]
#[ignore]
fn test_compilation_should_not_be_culture_dependent() {
    todo!("Not ported yet")
}

#[test]
#[ignore]
fn test_sources() {
    todo!("Not ported yet")
}
