use assert2::check;
use schema_registry_api::{Compatibility, CompatibilityLevel, ConfigClient, SubjectName};
use tracing::info;

pub async fn should_works_no_subject<'a>(client: &ConfigClient<'a>) -> anyhow::Result<()> {
    let compatibility = Compatibility {
        compatibility: CompatibilityLevel::Backward,
    };

    info!("configGet");
    let result = client.get().await?;
    check!(result.compatibility_level == compatibility.compatibility);

    info!("configSet");
    let result = client.set(compatibility).await?;
    check!(result == compatibility);

    Ok(())
}

pub async fn should_works_subject<'a>(
    client: &ConfigClient<'a>,
    name: &SubjectName,
) -> anyhow::Result<()> {
    let compatibility = Compatibility {
        compatibility: CompatibilityLevel::Backward,
    };

    info!("configSetSubject");
    let result = client.set_subject(name, compatibility).await?;
    check!(result == compatibility);

    info!("configGetSubject");
    let result = client.get_subject(name, None).await?;
    check!(result.compatibility_level == compatibility.compatibility);

    info!("configDeleteSubject");
    let result = client.delete_subject(name).await?;
    check!(result.compatibility_level == compatibility.compatibility);

    Ok(())
}
