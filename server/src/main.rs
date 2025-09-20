use glues_server::{parse_args, run};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = parse_args();
    run(args).await
}
