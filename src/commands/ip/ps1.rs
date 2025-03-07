use pnet::datalink::interfaces;

use crate::prelude::*;

/// Print network interface and ip part of the terminal shell's `$PS1`.
#[derive(clap::Parser, Debug)]
pub struct IpPs1Cmd {
    /// Don't show the network interface name.
    #[arg(long)]
    no_name: bool,
}

impl CliCommand for IpPs1Cmd {
    fn run(&self) -> CliResult {
        let mut outputs = vec![];
        let ifaces = interfaces();
        for v in ifaces.iter().filter(|&x| !(x.is_loopback() || x.ips.is_empty())) {
            let inner = v.ips.iter().filter(|&x| x.is_ipv4()).map(|x| {
                if self.no_name {
                    x.ip().to_string()
                } else {
                    format!("{}={}", v.name, x.ip())
                }
            });
            outputs.extend(inner);
        }
        println!("{}", outputs.join(" "));
        Ok(())
    }
}
