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
    let apperance = &config["apperance"];
    let org_name = config["orgName"].to_string().replace("\"", "");
    if org_name == "null" {
        utils::log_error("orgName is null!");
    }
    let mut dist = "<?xml version=\"1.0\" encoding=\"utf-8\" ?>".to_string();
    // basic config
    dist = format!("{}\n<installer-gui-script authoringTool=\"pak\" minSpecVersion=\"1.0\">", dist);
    dist = format!("{}\n<title>{}</title>", dist, project_name);
    dist = format!("{}\n<choices-outline>", dist);
    let mut tick = 0;
    dist = format!("{}\n<!--    Choice Data    -->", dist);
    for component_name in components.as_array().expect("Failed to unwrap object"){
        let component = &config[&format!("component_{}", component_name.to_string().replace("\"", ""))];
        let name = component["$name"].to_string().replace("\"", "");
        let pkg_name = component["$pkgName"].to_string().replace("\"", "");
        let payload_name = component["$payloadName"].to_string().replace("\"", "");
        let install_dir = component["$installDir"].to_string().replace("\"", "");
        let scripts_folder = &component["$scriptsFolder"].to_string().replace("\"", "");
        let mut extra_args = String::new();
        if name == "null" {
            utils::log_error("$name is null!");
        }
        if pkg_name == "null" {
            utils::log_error("$pkgName is null!");
        }
        if install_dir == "null" {
            utils::log_error("$installDir is null!");
        }
        if payload_name == "null" {
            utils::log_error("$payloadName is null!");
        }
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
        if scripts_folder != "null" {
            extra_args = format!("--scripts {}", scripts_folder).to_string();
        }
        Exec::shell(&format!("pkgbuild --identifier {}.{} --version {} --root {}/{} --quiet --install-location {} {} .build_cache/pkgs/{}.pkg", org_name, pkg_name, version, payload_dest_path.to_string(), payload_name, install_dir, extra_args, pkg_name)).join().unwrap();
        // Add choices
        dist = format!("{}\n<line choice=\"{}_install\" />", dist, tick.to_string());
        tick += 1;
    }
    dist = format!("{}\n<!--    End of Section    -->", dist);
    sp.message("Building Distribution".to_string());
    // close choices
    dist = format!("{}\n</choices-outline>", dist);
    tick = 0;
    // add choice tags
    dist = format!("{}\n<!--    Package Reference data and choices   -->", dist);
    for component_name in components.as_array().expect("Failed to unwrap object"){
        let component = &config[&format!("component_{}", component_name.to_string().replace("\"", ""))];
        let pkg_name = component["$pkgName"].to_string().replace("\"", "");
        let name = component["$name"].to_string().replace("\"", "");
        let mut desc = component["$desc"].to_string().replace("\"", "");
        let mut selected = component["$selected"].to_string().replace("\"", "");
        let mut visible = component["$visible"].to_string().replace("\"", "");
        let mut selectable = component["$selectable"].to_string().replace("\"", "");
        // enabled
        if name == "null" {
            utils::log_error("$name is null!");
        }
        if pkg_name == "null" {
            utils::log_error("$pkgName is null!");
        }
        if desc == "null" {
            desc = "".to_string();
        }
        if selected == "null" {
            // default is true
            selected = "true".to_string();
        }
        if visible == "null" {
            // default is true
            visible = "true".to_string();
        }
        if selectable == "null" {
            // default is true
            selectable = "true".to_string();
        }
        dist = format!("{}\n<choice id=\"{tick}_install\" title=\"{name}\" description=\"{desc}\" start_selected=\"{selected}\" visible=\"{visible}\" enabled=\"{selectable}\">\n<pkg-ref id=\"{tick}_installer\" />\n</choice>", dist, tick = tick, name = name, desc = desc, selected = selected, visible = visible, selectable = selectable);
        dist = format!("{}\n<pkg-ref id=\"{}_installer\" version=\"1.0\" auth=\"Root\">{}.pkg</pkg-ref>", dist, tick, pkg_name);
        tick += 1;
    }
    dist = format!("{}\n<!--    End of Section    -->", dist);
    // add apperance data
    dist = format!("{}\n<!--    Apperance Data    -->", dist);
    match fs::create_dir_all(".build_cache/resources/"){
        Ok(_) => print!(""),
        Err(err) => {
            utils::log_error(&err.to_string());
        }
    }
    if apperance != "null" {
        // update apperance config
        let readme = apperance["$readme"].to_string().replace("\"", "");
        let license = apperance["$license"].to_string().replace("\"", "");
        let welcome_html = apperance["$welcomeHtml"].to_string().replace("\"", "");
        let complete_html = apperance["$conclusionHtml"].to_string().replace("\"", "");
        let background_config = &apperance["$backgroundConfig"];
        if readme != "null" {
            sp.message("Updating apperance config for: README".to_string());
            utils::require_path(readme.to_string());
            utils::copy_file(&readme.to_string(), ".build_cache/resources/README.txt");
            dist = format!("{}\n<readme file=\"README.txt\" mime-type=\"text/plain\" />", dist);
        }
        if license != "null" {
            sp.message("Updating apperance config for: license".to_string());
            utils::require_path(license.to_string());
            utils::copy_file(&license.to_string(), ".build_cache/resources/license.txt");
            dist = format!("{}\n<license file=\"license.txt\" />", dist);
        }
        if welcome_html != "null" {
            sp.message("Updating apperance config for: welcome.html".to_string());
            utils::require_path(welcome_html.to_string());
            utils::copy_file(&welcome_html.to_string(), ".build_cache/resources/welcome.html");
            dist = format!("{}\n<welcome file=\"welcome.html\" mime-type=\"text/html\" />", dist);
        }
        if complete_html != "null" {
            sp.message("Updating apperance config for: complete.html".to_string());
            utils::require_path(complete_html.to_string());
            utils::copy_file(&complete_html.to_string(), ".build_cache/resources/complete.html");
            dist = format!("{}\n<conclusion file=\"complete.html\" mime-type=\"text/html\" />", dist);
        }
        if background_config != "null" {
            let image_file = background_config["$imageFile"].to_string().replace("\"", "");
            let align = background_config["$align"].to_string().replace("\"", "");
            if image_file == "null" {
                utils::log_error("imageFile must have a value!");
            }
            if align == "null" {
                utils::log_error("align must have a value!");
            }
            utils::require_path(image_file.to_string());
            utils::copy_file(&image_file, ".build_cache/resources/background.png");
            sp.message("Updating apperance config for: background".to_string());
            dist = format!("{}\n<background file=\"background.png\" alignment=\"{}\" />", dist, align);
        }
    }
    // end xml
    dist = format!("{}\n<!--    End of Section    -->", dist);
    dist = format!("{}\n</installer-gui-script>", dist);
    fs::write(".build_cache/Distfile", dist).expect("Failed to write dist file");
    sp.message("Building final package".to_string());
    Exec::shell(&format!("productbuild --quiet --resources .build_cache/resources --distribution .build_cache/Distfile --package-path .build_cache/pkgs builds/out.pkg")).join().expect("Failed to build final package");
    sp.message("Built Package".to_string());
    sp.stop();
}