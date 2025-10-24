use tracing_subscriber::EnvFilter;

pub fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::from_default_env()
        .add_directive("s3fs=debug".parse()?)
        .add_directive("s3s=debug".parse()?)
        .add_directive("s3s_fs=debug".parse()?)
        .add_directive("hyper=debug".parse()?);

    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    Ok(())
}
