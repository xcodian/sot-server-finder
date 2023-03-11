use std::{net::IpAddr, process::Command};

use etherparse::{IpHeader, PacketHeaders};
use sysinfo::{PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};
use winroute::{Route, RouteManager};

fn get_sot_pid(s: &System) -> Option<u32> {
    for process in s.processes_by_name("SoTGame.exe") {
        return Some(process.pid().as_u32());
    }

    None
}

fn get_sot_ports(pid: u32) -> Vec<u16> {
    let p = &pid.to_string();

    let cmd = Command::new("netstat")
        .arg("-anop")
        .arg("udp")
        .output()
        .unwrap();

    // jarringly, netstat output contains non-utf8 characters :)
    let filtered_stdout = cmd
        .stdout
        .iter()
        .filter(|c| c.is_ascii())
        .copied()
        .collect();

    String::from_utf8(filtered_stdout)
        .unwrap()
        .lines()
        .filter(|line| line.contains(p))
        .map(|f| {
            let addr = f.split_whitespace().skip(1).next().unwrap();
            let port = addr.split(':').last().unwrap();
            port.parse::<u16>().unwrap()
        })
        .collect()
}

fn main() {
    println!("Making sure you have Npcap installed...");
    unsafe {
        let try_load_wpcap = libloading::Library::new("wpcap.dll");
        if try_load_wpcap.is_err() {
            println!("{}", "*".repeat(80));
            println!("ERROR: It doesn't seem like you've installed Npcap.");
            println!("Please install Npcap from\n    https://npcap.com/dist/npcap-1.72.exe\n");
            println!("*** MAKE SURE TO INSTALL WITH 'WinPcap API Compatibility' TURNED ON ***");
            println!("{}\n", "*".repeat(80));
            println!("Want to continue anyway? Enter 'yes' or 'no':");
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();
            if !(input == "y" || input == "yes") {
                std::process::exit(1);
            }
        }
    }

    // wait until we get a sot pid
    println!("Waiting for Sea of Thieves to be running... (you should start it)");
    let mut s =
        System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::new()));

    let sot_pid = loop {
        if let Some(pid) = get_sot_pid(&s) {
            break pid;
        }
        s.refresh_processes();
    };

    println!("Found! PID: {}", sot_pid);

    let devices = pcap::Device::list().unwrap();
    let auto_found_dev = devices.iter().find(|d| {
        d.addresses.iter().any(|addr| {
            if let IpAddr::V4(addr) = addr.addr {
                addr.octets()[0] == 192 && addr.octets()[1] == 168
            } else {
                false
            }
        })
    });

    let dev = match auto_found_dev {
        Some(d) => d.clone(),
        None => {
            println!("Couldn't guess which network adapter to use. Please select one manually.");
            println!("Network adapters attached to your PC: ");

            let devices = pcap::Device::list().expect("device lookup failed");
            let mut i = 1;

            for device in devices.clone() {
                println!(
                    "    {i}. {:?}",
                    device.desc.clone().unwrap_or(device.name.clone())
                );
                i += 1;
            }

            // prompt user for their device
            println!(
                "Please select your WiFi or Ethernet card, or if you're on a VPN, select the VPN: "
            );
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let n = input.trim().parse::<usize>().unwrap() - 1;

            (&devices[n]).clone()
        }
    };

    let mut cap = pcap::Capture::from_device(dev)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    let route_manager = RouteManager::new().unwrap();
    let the_void = "0.0.0.0".parse().unwrap();

    println!("Which server are you trying to connect to? (e.g. 20.213.146.107:30618)\n    Enter 'idk' if you want to just print the server you're connecting to.");
    let mut target = String::new(); // ""
    std::io::stdin().read_line(&mut target).unwrap();
    let target = target.trim();

    if target == "idk" {
        println!("Alright, will print connected server.");
    } else {
        println!("Alright, server hop target: {}", target);
    }

    println!("Waiting for you to connect to a game in Sea of Thieves...");

    // iterate udp packets
    loop {
        if let Ok(raw_packet) = cap.next_packet() {
            if let Ok(packet) = PacketHeaders::from_ethernet_slice(raw_packet.data) {
                if let Some(IpHeader::Version4(ipv4, _)) = packet.ip {
                    if let Some(transport) = packet.transport {
                        if let Some(udp) = transport.udp() {
                            if udp.destination_port == 3075 || udp.destination_port == 30005 {
                                continue;
                            }

                            if get_sot_ports(sot_pid).contains(&udp.source_port) {
                                let ip = ipv4.destination.map(|c| c.to_string()).join(".");

                                if target == "idk" {
                                    println!("You are connected to: {}:{}\n   Press Enter to check again.", ip, udp.destination_port);
                                    std::io::stdin().read_line(&mut String::new()).unwrap();
                                    continue;
                                }

                                if format!("{}:{}", ip, udp.destination_port) != target {
                                    println!(
                                        "FAIL {}:{}, not the right server.",
                                        ip, udp.destination_port
                                    );
                                } else {
                                    println!("SUCCESS {}:{}", ip, udp.destination_port);
                                    std::io::stdin().read_line(&mut String::new()).unwrap();
                                    break;
                                }

                                let blocking_route =
                                    Route::new(ip.parse().unwrap(), 32).gateway(the_void);

                                // add route
                                if let Err(e) = route_manager.add_route(&blocking_route) {
                                    println!(
                                        "Error adding route for: {}:{} - {}",
                                        ip, udp.destination_port, e
                                    );
                                } else {
                                    // wait for enter
                                    println!("Answer no to 'Do you want to rejoin your previous session?', then press Enter here.");
                                    std::io::stdin().read_line(&mut String::new()).unwrap();
                                }

                                println!("Unblocking {}...", ip);

                                // delete route, route_manager.delete_route doesn't work for some reason
                                let status = Command::new("route")
                                    .arg("delete")
                                    .arg(ip)
                                    .status()
                                    .unwrap();
                                if !status.success() {
                                    println!("Failed to delete route.");
                                }

                                println!("Try setting sail again.");
                            }
                        }
                    }
                }
            }
        }
    }
}
