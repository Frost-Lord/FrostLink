use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::time::Instant;
use std::fs::{File, read_to_string};
use std::io::BufReader;
use std::path::Path;
use rustls::{
    ServerConfig, NoClientAuth, ResolvesServerCertUsingSNI, 
    internal::pemfile::{certs, pkcs8_private_keys},
    sign::{CertifiedKey, RSASigningKey}
};
use tokio_rustls::TlsAcceptor;
use crate::BColors;
use crate::components;

pub async fn handle_client(configs: Arc<Mutex<Vec<(String, bool, String, String, String)>>>, colors: BColors, client_stream: TcpStream) -> std::io::Result<()> {
    let start_time = Instant::now();

    let mut buffer = vec![0; 1024];
    let mut rustls_config = ServerConfig::new(NoClientAuth::new());

    let mut resolver = ResolvesServerCertUsingSNI::new();
    for (domain, _, _, ssl_certificate, ssl_certificate_key) in configs.lock().await.iter() {
        let cert_file = &mut BufReader::new(File::open(Path::new(ssl_certificate))?);
        let key_file = &mut BufReader::new(File::open(Path::new(ssl_certificate_key))?);

        let cert_chain = match certs(cert_file) {
            Ok(c) if !c.is_empty() => c,
            _ => {
                eprintln!("{}[ARCTICARCH]{} Failed to parse certs for domain {}", colors.fail, colors.endc, domain);
                continue;
            }
        };

        let keys = match pkcs8_private_keys(key_file) {
            Ok(k) if !k.is_empty() => k,
            _ => {
                eprintln!("{}[ARCTICARCH]{} Failed to parse keys for domain {}", colors.fail, colors.endc, domain);
                continue;
            }
        };

        let key = match RSASigningKey::new(&keys[0]) {
            Ok(key) => key,
            Err(_) => {
                eprintln!("{}[ARCTICARCH]{} Failed to create signing key for domain {}", colors.fail, colors.endc, domain);
                continue;
            }
        };
        let signing_key = CertifiedKey::new(cert_chain, Arc::new(Box::new(key)));

        if resolver.add(domain, signing_key).is_err() {
            eprintln!("{}[ARCTICARCH]{} Failed to add certificate for domain {}", colors.fail, colors.endc, domain);
        }
    }

    rustls_config.cert_resolver = Arc::new(resolver);

    let tls_acceptor = TlsAcceptor::from(Arc::new(rustls_config));
    let mut tls_stream = tls_acceptor.accept(client_stream).await?;

    let size = tls_stream.read(&mut buffer).await?;
    let request_str = String::from_utf8_lossy(&buffer[0..size]);
    let request_path = request_str.split_whitespace().nth(1).unwrap_or("/").to_string();

    let domain_and_location = {
        //eprintln!("{}[ARCTICARCH]{} Request path: {:?}", colors.fail, colors.endc, request_str.lines());
        let host_header = request_str.lines()
        .find(|line| line.starts_with("Host:"))
        .and_then(|line| line.splitn(2, ':').nth(1))
        .and_then(|host| host.split_whitespace().next())
        .map(|host| host.to_string());

        //eprintln!("{}[ARCTICARCH]{} Host header: {:?}", colors.fail, colors.endc, host_header);
        let configs = configs.lock().await;
        let found_domain = configs.iter().find(|(domain, _, _, _, _)| {
            host_header.as_ref() == Some(&domain)
        });
        //eprintln!("{}[ARCTICARCH]{} Found domain: {:?}", colors.fail, colors.endc, found_domain);
        found_domain.cloned()
    };

    if let Some((domain, _, location, ssl_certificate, ssl_certificate_key)) = domain_and_location {
        let mut rustls_config = ServerConfig::new(NoClientAuth::new());
        let cert_file = components::load_cert(Path::new(&ssl_certificate))?;
        let key_file = components::load_private_key(Path::new(&ssl_certificate_key))?;
        rustls_config.set_single_cert(cert_file, key_file).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        if let Ok(mut remote_stream) = TcpStream::connect(&location).await {
        if remote_stream.write(&buffer[0..size]).await.is_ok() {
            if io::copy(&mut remote_stream, &mut tls_stream).await.is_ok() {
                let ip = tls_stream.get_ref().0.peer_addr().unwrap().ip();
                let elapsed_time = start_time.elapsed();
                println!("Time taken: {:?} : {} : {} : {} : SSL: True", elapsed_time, ip, domain, request_path);
            } else {
                eprintln!("{}[ARCTICARCH]{} Failed to send response to client", colors.fail, colors.endc);
            }
            } else {
                eprintln!("{}[ARCTICARCH]{} SSL Failed to send request to {}", colors.fail, colors.endc, location);
            }
        } else {
            eprintln!("{}[ARCTICARCH]{} Could not connect to {}", colors.fail, colors.endc, location);
        }
    } else {
        let default_file_path = "./default/index.html";
        match read_to_string(default_file_path) {
            Ok(contents) => {
                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", contents);
                tls_stream.write_all(response.as_bytes()).await?;
            }
            Err(_) => {
                let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\n\r\n<html><head><title>Not Found</title></head><body><h1>404 - File Not Found</h1></body></html>";
                tls_stream.write_all(response.as_bytes()).await?;
                eprintln!("{}[ARCTICARCH]{} Failed to read the default index.html file from {}", colors.fail, colors.endc, default_file_path);
            }
        }
    }

    Ok(())
}
