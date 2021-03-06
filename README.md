# Astro DNS-SD

[![Build Status](https://dev.azure.com/AstroHQ/astro-dnssd/_apis/build/status/AstroHQ.astro-dnssd?branchName=master)](https://dev.azure.com/AstroHQ/astro-dnssd/_build/latest?definitionId=1&branchName=master)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/AstroHQ/astro-dnssd)
[![Cargo](https://img.shields.io/crates/v/astro-dnssd.svg)](https://crates.io/crates/astro-dnssd)
[![Documentation](https://docs.rs/astro-dnssd/badge.svg)](https://docs.rs/astro-dnssd)

Minimal but friendly safe wrapper around dns-sd(Bonjour, mDNS, Zeroconf DNS) APIs.

[Documentation](https://crates.io/crates/astro-dnssd)

## Features

### Complete

- Service registration
- TXTRecord support for service registration

### In Progress

- Service browsing

### Todo

- How to check for more (select() on socket, but has to be win32 friendly)
- Record creation
- Name resolution
- Port map
- Tests
- Documentation
- Pure Rust TXT code?
- Interior mutability? (Can we reduce the &mut arguments some?)

## Build Requirements
`astro-dnssd` requires the Bonjour SDK.

- **Windows:** Download the SDK [here]( https://developer.apple.com/bonjour/)
- **Linux:** Install `avahi-compat-libdns_sd` for your distro of choice.

## Technical Background
This [website](http://www.dns-sd.org/) provides a good overview of the DNS-SD protocol.

## Example

```rust
    use astro_dnssd::register::DNSServiceBuilder;
    use astro_dnssd::txt::TXTRecord;
    let mut txt = TXTRecord::new();
    let _ = txt.insert("s", Some("open"));
    let mut service = DNSServiceBuilder::new("_rust._tcp")
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
        service.process_result();
    }
```

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
