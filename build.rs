use std::{io::Cursor, fs};

fn main() {
    // refuse to build if we're not on windows
    if !cfg!(target_os = "windows") {
        panic!("This crate only works on Windows!");
    }

    let mut res = winres::WindowsResource::new();
    res.set_manifest(r#"
    <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
            </requestedPrivileges>
        </security>
    </trustInfo>
    </assembly>
    "#);
    res.compile().unwrap();

    // create the libs folder if it doesn't exist
    let libs = std::path::Path::new("libs");
    if !libs.exists() {
        fs::create_dir("libs").unwrap();

        // download the zip from https://npcap.com/dist/npcap-sdk-1.13.zip
        let resp = reqwest::blocking::get("https://npcap.com/dist/npcap-sdk-1.13.zip")
            .expect("Failed to download npcap-sdk-1.13.zip");

        let zip = resp.bytes().unwrap();

        zip_extract::extract(Cursor::new(zip), &libs.join("zip"), false).unwrap();
        fs::copy("libs/zip/Lib/x64/wpcap.lib", "libs/wpcap.lib").unwrap();
        fs::copy("libs/zip/Lib/x64/Packet.lib", "libs/Packet.lib").unwrap();
        fs::remove_dir_all("libs/zip").unwrap();
        fs::remove_dir("libs/zip").unwrap();
    }

    println!(r#"cargo:rustc-env=LIB=libs\"#);
}
