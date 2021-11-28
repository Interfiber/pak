pub fn execute_pak_cmd(args: &str){
    subprocess::Exec::shell(&format!("pakcli {}", args)).join().expect("Failed to execute pak command");
}