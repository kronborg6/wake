use bollard::Docker;

pub async fn version() -> Result<(), bollard::errors::Error> {
    // let docker = Docker::connect_with_defaults().unwrap();

    let connection = Docker::connect_with_defaults().unwrap();
    let gg = connection.ping().await?;
    let version = connection.version().await?;

    println!("ping: {}", gg);
    println!("version: {:?}", version);

    Ok(())
}
