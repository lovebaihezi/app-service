use actix_web::{main, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

fn ssl_init(key_pem: &'static str, cert_pem: &'static str) -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(key_pem, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(cert_pem).unwrap();
    builder
}

#[main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        tracing_subscriber::fmt::init();
    }
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8888".into())
        .parse::<u16>()
        .unwrap();
    let addr = format!(
        "{}:{}",
        std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        port
    );
    let mut server = HttpServer::new(|| App::new());
    let server = if cfg!(debug_assertions) {
        let builder = ssl_init("../key.pem", "../cert_pem");
        server.bind_openssl(addr, builder)?
    } else {
        server.bind(addr)?
    };
    server.run().await?;
    Ok(())
}
