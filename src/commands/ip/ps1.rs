use clap;
use pnet::datalink::interfaces;

use crate::prelude::*;

/// Print network interface and ip part of the terminal shell's `$PS1`.
#[derive(clap::Parser, Debug)]
pub struct IpPs1Cmd {}

impl CliCommand for IpPs1Cmd {
    fn run(&self) -> CliCommandResult {
        let mut outputs = vec![];
        let ifaces = interfaces();
        for v in &ifaces {
            if v.is_loopback() || v.ips.is_empty() {
                continue;
            }
            for ip in &v.ips {
                if ip.is_ipv4() {
                    outputs.push(format!("{}={}", v.name, ip.ip()));
                }
            }
        }
        println!("{}", outputs.join(" "));
        Ok(())
    }
}
