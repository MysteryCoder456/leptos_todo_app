fn main() {
    // Rebuild Tailwind CSS code

    // Rebuild only if CSS has changed
    println!("cargo:rerun-if-changed=style/input.css");

    // Calling Tailwind CLI
    let output = std::process::Command::new("tailwind")
        .arg("-i")
        .arg("./style/input.css")
        .arg("-o")
        .arg("./public/output.css")
        .output()
        .expect("Failed to run Tailwind build");

    let output = if output.status.success() {
        output.stdout
    } else {
        output.stderr
    };

    println!(
        "{}",
        output.iter().map(|c| char::from(*c)).collect::<String>()
    );
}
