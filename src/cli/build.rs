use spinners::{Spinner, Spinners};
use crate::utils;
use serde_json::Value;
use std::fs;
use fs_extra::dir::copy_with_progress;
use fs_extra::dir::CopyOptions;
use fs_extra::dir::TransitProcess;
use subprocess::Exec;


pub fn build(){
    let build_cache_name = ".build_cache";
    let project_file_name = "pak.project.json";

    let sp = Spinner::new(&Spinners::Dots12, "Building Package".into());
    utils::create_dir(build_cache_name);
    utils::create_dir(".build_cache/pkgs");
    sp.message("Loading config...".to_string());
    // Load config
    let json_data = fs::read_to_string(project_file_name).expect("Failed to read contents of file into memory!");
    let config: Value = serde_json::from_str(&json_data).expect("Failed to parse json file!");
    // Get values
    let project_name = config["projectName"].to_string().replace("\"", "");
    let version = config["version"].to_string().replace("\"", "");
    let components = &config["components"];
    let org_name = config["orgName"].to_string().replace("\"", "");
    if org_name == "null" {
        println!("ERROR! Failed to complete build!");
        println!("Error: orgName is null");
        std::process::exit(1);
    }
    let mut dist = "<?xml version=\"1.0\" encoding=\"utf-8\" ?>".to_string();
    // basic config
    dist = format!("{}\n<installer-gui-script authoringTool=\"pak\" minSpecVersion=\"1.0\">", dist);
    dist = format!("{}\n<choices-outline>", dist);
    let mut tick = 0;
    for component_name in components.as_array().expect("Failed to unwrap object"){
        let component = &config[&format!("component_{}", component_name.to_string().replace("\"", ""))];
        let name = component["$name"].to_string().replace("\"", "");
        let pkg_name = component["$pkgName"].to_string().replace("\"", "");
        let payload_name = component["$payloadName"].to_string().replace("\"", "");
        let install_dir = component["$installDir"].to_string().replace("\"", "");
        sp.message(format!("Building: {}", name));
        utils::create_dir(&format!(".build_cache/{}_temp", pkg_name));
        // Copy payload
        // Get payload path
        let payload_src_path = format!("payloads/{}", payload_name);
        let payload_dest_path = format!(".build_cache/{}_temp", pkg_name);
        utils::require_path(payload_src_path.to_string());
        let options = CopyOptions::new(); //Initialize default values for CopyOptions
        let handle = |process_info: TransitProcess|  {
            sp.message(format!("Copying payload: {}/{} bytes", process_info.copied_bytes, process_info.total_bytes));
            fs_extra::dir::TransitProcessResult::ContinueOrAbort
        };
        copy_with_progress(payload_src_path, &format!("{}", payload_dest_path), &options, handle).expect("Failed to copy payload");
        sp.message("Building package component...".to_string());
        Exec::shell(&format!("pkgbuild --identifier {}.{} --version 0.0.1 --root {}/{} --quiet --install-location {} .build_cache/pkgs/{}.pkg", org_name, pkg_name, payload_dest_path.to_string(), payload_name, install_dir, pkg_name)).join().unwrap();
        // Add choices
        dist = format!("{}\n<line choice=\"{}_install\" />", dist, tick.to_string());
        tick += 1;
    }
    sp.message("Building Distribution".to_string());
    // close choices
    dist = format!("{}\n</choices-outline>", dist);
    tick = 0;
    // add choice tags
    for component_name in components.as_array().expect("Failed to unwrap object"){
        let component = &config[&format!("component_{}", component_name.to_string().replace("\"", ""))];
        let pkg_name = component["$pkgName"].to_string().replace("\"", "");
        let name = component["$name"].to_string().replace("\"", "");
        dist = format!("{}\n<choice id=\"{}_install\" title=\"{}\">\n<pkg-ref id=\"{}_installer\" />\n</choice>", dist, tick, name, tick);
        dist = format!("{}\n<pkg-ref id=\"{}_installer\" version=\"1.0\" auth=\"Root\">{}.pkg</pkg-ref>", dist, tick, pkg_name);
        tick += 1;
    }
    // end xml
    dist = format!("{}\n</installer-gui-script>", dist);
    fs::write(".build_cache/Distfile", dist).expect("Failed to write dist file");
    sp.message("Building final package".to_string());
    Exec::shell(&format!("productbuild --quiet --distribution .build_cache/Distfile --package-path .build_cache/pkgs builds/out.pkg")).join().expect("Failed to build final package");
    sp.stop();
    println!("Built package.");
}