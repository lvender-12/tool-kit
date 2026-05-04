use std::{sync::Arc, time::Duration};

use tokio::{net::TcpStream, sync::Semaphore, time::timeout};

use crate::utils::util;

pub async fn run() {
    let addr = util::question("IP/URL ");
    let port_start = util::question("Port Start ").parse::<u16>().unwrap();
    let port_end = util::question("Port End ").parse::<u16>().unwrap();

    let max_current = 200;

    scan_ports(&addr, port_start, port_end, max_current).await
}

async fn scan_ports(ip: &str, start: u16, end: u16, limit: usize) {
    let sem = Arc::new(Semaphore::new(limit));

    let mut handles = vec![];

    for port in start..=end {
        let addr = format!("{}:{}", ip, port);
        let permit = sem.clone().acquire_owned().await.unwrap();
        let handle = tokio::spawn(async move {
            let result = timeout(Duration::from_millis(500), TcpStream::connect(&addr)).await;

            drop(permit);

            match result {
                Ok(_) => {
                    let service = port_to_service(port);
                    println!("Port {} is open ({})", port, service)
                }
                _ => {}
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

fn port_to_service(port: u16) -> &'static str {
    match port {
        // ───── Core Internet Services ─────
        21 => "FTP",
        22 => "SSH",
        23 => "Telnet",
        25 => "SMTP",
        53 => "DNS",
        67 => "DHCP Server",
        68 => "DHCP Client",
        69 => "TFTP",

        // ───── Web ─────
        80 => "HTTP",
        443 => "HTTPS",
        8080 => "HTTP-Proxy / Alt HTTP",
        8443 => "HTTPS-Alt",

        // ───── Email ─────
        110 => "POP3",
        143 => "IMAP",
        465 => "SMTP SSL",
        587 => "SMTP Submission",
        993 => "IMAP SSL",
        995 => "POP3 SSL",

        // ───── Database ─────
        3306 => "MySQL",
        5432 => "PostgreSQL",
        1521 => "Oracle DB",
        6379 => "Redis",
        27017 => "MongoDB",

        // ───── Remote / Admin ─────
        3389 => "RDP (Remote Desktop)",
        5900 => "VNC",
        5985 => "WinRM HTTP",
        5986 => "WinRM HTTPS",

        // ───── Dev / Services ─────
        5000 => "Flask / Dev Server",
        8000 => "HTTP Dev Server",
        8001 => "HTTP Alt Dev",

        // ───── Hosting / cPanel / WHM (yang kamu lihat tadi) ─────
        2082 => "cPanel",
        2083 => "cPanel SSL",
        2086 => "WHM",
        2087 => "WHM SSL",
        2095 => "Webmail",
        2096 => "Webmail SSL",

        // ───── Proxy / Misc Web ─────
        3128 => "Squid Proxy",
        8888 => "HTTP Proxy / Dev Server",
        9000 => "Web Admin / Dev Tool",

        // ───── File Sharing ─────
        139 => "SMB",
        445 => "SMB Direct",
        2049 => "NFS",

        // ───── Default ─────
        _ => "UNKNOWN / Custom Service",
    }
}
