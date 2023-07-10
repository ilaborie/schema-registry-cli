use std::path::Path;

use assert2::{assert, check, let_assert};
use clap_complete::Shell;
use schema_registry_api::{SchemaId, SchemaVersion, SubjectName};
use schema_registry_cli::command::{
    check_compatibility, delete_subject, get_schema, list_subjects, register_schema,
};
use schema_registry_cli::{
    init_tracing, process, CheckCompatibility, Command, Completion, DeleteSubject, GetSchema,
    RegisterSchemaSettings, SchemaRegistrySettings, SchemaSubCommand, Settings, SubjectSubCommand,
    Verbosity,
};
use ulid::Ulid;

#[tokio::test]
async fn should_api_works() -> anyhow::Result<()> {
    let _ = init_tracing(Verbosity::INFO);

    let client_settings = SchemaRegistrySettings::default();

    // Lists subjects
    let all = false;
    let prefix = None;
    let subjects = list_subjects(client_settings.clone(), all, prefix).await?;
    dbg!(&subjects);

    // Register schema
    let subject = format!("my-test-subject-{}", Ulid::new());
    let subject = subject.parse::<SubjectName>()?;
    let path = Path::new("tests/assets/a_record.avsc");
    let id = register_schema(client_settings.clone(), Some(subject.clone()), &path, false).await?;
    dbg!(&id);

    // Check compatibility
    let version = Some(SchemaVersion::Latest);
    check_compatibility(
        client_settings.clone(),
        Some(subject.clone()),
        &path,
        version,
        true,
    )
    .await?;
    let compat = check_compatibility(
        client_settings.clone(),
        Some(subject.clone()),
        &path,
        None,
        true,
    )
    .await?;
    dbg!(&compat);
    assert!(compat.is_compatible);

    // Get schema
    let _ = get_schema(client_settings.clone(), id, Some(&subject)).await?;
    let schema = get_schema(client_settings.clone(), id, None).await?;
    dbg!(&schema);
    let_assert!(Some(sc) = schema);
    check!(sc.schema == include_str!("./assets/a_record.avsc").replace(char::is_whitespace, ""));

    // Delete subject
    let deleted1 = delete_subject(client_settings.clone(), &subject, None, false).await?;
    dbg!(&deleted1);
    let deleted2 = delete_subject(client_settings, &subject, version, true).await?;
    dbg!(&deleted2);

    Ok(())
}

#[tokio::test]
async fn should_lib_works() -> anyhow::Result<()> {
    let _ = init_tracing(Verbosity::INFO);

    let client_settings = SchemaRegistrySettings::default();
    let verbosity = Verbosity::default();

    // Completions

    process(Settings {
        verbosity,
        command: Command::Completion(Completion {
            shell: Shell::Bash,
            out_dir: None,
        }),
    })
    .await?;

    // Lists subjects
    let command = SubjectSubCommand::default();
    process(Settings {
        verbosity,
        command: Command::Subject(command),
    })
    .await?;

    // Register schema
    let subject = format!("my-test-subject-{}", Ulid::new());
    let subject = subject.parse::<SubjectName>()?;
    let path = Path::new("tests/assets/a_record.avsc");

    let command = SubjectSubCommand::Register(RegisterSchemaSettings {
        schema_registry: client_settings.clone(),
        subject: Some(subject.clone()),
        normalize: true,
        path: path.to_path_buf(),
    });
    process(Settings {
        verbosity,
        command: Command::Subject(command),
    })
    .await?;

    // Check compatibility
    let command = SubjectSubCommand::Check(CheckCompatibility {
        schema_registry: client_settings.clone(),
        subject: Some(subject.clone()),
        version: None,
        path: path.to_path_buf(),
    });
    process(Settings {
        verbosity,
        command: Command::Subject(command),
    })
    .await?;

    // Get schema
    let command = SchemaSubCommand::Get(GetSchema {
        schema_registry: client_settings.clone(),
        subject: None,
        id: SchemaId::from(1),
    });
    process(Settings {
        verbosity,
        command: Command::Schema(command),
    })
    .await?;

    // Delete subject
    let command = SubjectSubCommand::Delete(DeleteSubject {
        schema_registry: client_settings.clone(),
        subject: subject.clone(),
        version: None,
        permanent: false,
    });
    process(Settings {
        verbosity,
        command: Command::Subject(command),
    })
    .await?;

    Ok(())
}
