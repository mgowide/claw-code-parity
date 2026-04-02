use web_server::{default_addr, serve};

#[tokio::main]
async fn main() {
    let addr = default_addr();
    if let Err(e) = serve(addr).await {
        eprintln!("server error: {e}");
        std::process::exit(1);
    }
}
