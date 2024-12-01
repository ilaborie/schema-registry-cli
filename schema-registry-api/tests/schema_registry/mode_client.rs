use assert2::check;
use schema_registry_api::{Mode, ModeClient, SubjectName};
use tracing::info;

pub async fn should_works_no_subject<'a>(client: &ModeClient<'a>) -> anyhow::Result<()> {
    let mode = Mode::default();

    info!("modeGet");
    let result = client.get().await?;
    check!(result == mode);

    info!("modeSet");
    let result = client.set(mode, None).await?;
    check!(result == mode);

    Ok(())
}

pub async fn should_works_subject<'a>(
    client: &ModeClient<'a>,
    name: &SubjectName,
) -> anyhow::Result<()> {
    let mode = Mode::default();

    info!("modeSetSubject");
    let result = client.set_subject(name, mode, None).await?;
    check!(result == mode);

    info!("modeGetSubject");
    let result = client.get_subject(name).await?;
    check!(result == mode);

    info!("modeDeleteSubject");
    let result = client.delete_subject(name).await?;
    check!(result == mode);

    Ok(())
}
