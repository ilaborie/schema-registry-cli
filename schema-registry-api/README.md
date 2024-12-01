# Schema registry API

[![Crates.io](https://img.shields.io/crates/v/schema-registry-api.svg)](https://crates.io/crates/schema-registry-api)
[![Documentation](https://docs.rs/schema-registry-api/badge.svg)](https://docs.rs/schema-registry-api/)
[![Codecov](https://codecov.io/github/ilaborie/schema-registry-api/coverage.svg?branch=main)](https://codecov.io/gh/ilaborie/schema-registry-api)
[![Dependency status](https://deps.rs/repo/github/ilaborie/schema-registry-api/status.svg)](https://deps.rs/repo/github/ilaborie/schema-registry-api)

Provide a REST API to call with a schema registry.

## Examples

List subjects

```rust, no_run
use reqwest::Url;
use schema_registry_api::SchemaRegistry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a `SchemaRegistry`
    let base_url = Url::parse("http://localhost:8081")?;
    let sr = SchemaRegistry::build_default(base_url)?;

    // List subjects
    let subjects = sr.subject().list(None, None).await?;

    if subjects.is_empty() {
        println!("No subject found");
    }
    for subject in &subjects {
        println!("Found subject '{subject}'");
    }
    Ok(())
}
```

Register a schema

```rust, no_run
use reqwest::Url;
use schema_registry_api::{RegisterSchema, SchemaRegistry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a `SchemaRegistry`
    let base_url = Url::parse("http://localhost:8081")?;
    let sr = SchemaRegistry::build_default(base_url)?;

    // Create a subject
    let subject = "a-topic-value".parse()?;
    // Create the `RegisterSchema`
    let schema = RegisterSchema {
        schema: r#"
{
    "type": "record",
    "name": "test",
    "fields": [
        {
            "name": "field1",
            "type": "string"
        }
    ]
}"#.to_string(),
        ..Default::default()
    };

    // Register the schema for the subject
    let registered_schema = sr.subject().new_version(&subject, &schema, None).await?;
    // Get the schema id
    let schema_id = registered_schema.id;

    println!("The schema #{schema_id} is registered for subject '{subject}'");
    Ok(())
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
