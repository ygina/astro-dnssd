use astro_dnssd::register::DNSServiceBuilder;
use astro_dnssd::txt::TXTRecord;

fn main() {
    println!("Registering service...");
    let mut txt = TXTRecord::new();
    let _ = txt.insert("status", Some("open"));
    let mut service = DNSServiceBuilder::new("_http._tcp")
        .with_port(2048)
        .with_name("MyRustService")
        .with_txt_record(txt)
        .build()
        .unwrap();
    let _result = service.register(|reply| match reply {
        Ok(reply) => println!("Successful reply: {:?}", reply),
        Err(e) => println!("Error registering: {:?}", e),
    });
    loop {
        if service.has_data() {
            service.process_result();
        }
    }
}
