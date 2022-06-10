use std::process::Command;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use cli_clipboard::linux_clipboard::LinuxClipboardContext;

pub fn get_ssh_pub_key() -> String {

    let home_path= match home::home_dir() {
        Some(path) => path.display().to_string(),
        None => panic!("No se puede obtener el directorio raíz."),
    };

    let ssh_key_pub= Command::new("cat")
        .arg(format!("{}/.ssh/id_rsa.pub", &home_path))
        .output()
        .unwrap();
    let ssh_key_pub= String::from_utf8(ssh_key_pub.stdout).unwrap();

    let ssh_key_pub= ssh_key_pub.trim().to_string();
    ssh_key_pub.replace("\n", "")
}

pub fn copy_ssh_pub_key_to_clipboard(){
    cli_clipboard::set_contents(get_ssh_pub_key().to_string());
    println!("{}", cli_clipboard::get_contents().unwrap());
}