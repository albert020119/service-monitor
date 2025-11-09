use crate::models::service::Service;
use trust_dns_resolver::TokioAsyncResolver;

pub async fn run(service: &Service) {
    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();
    let result = resolver.lookup_ip(service.url.as_str()).await;

    match result {
        Ok(_) => println!("{} DNS OK", service.name),
        Err(e) => println!("{} DNS FAILED: {}", service.name, e),
    }
}
