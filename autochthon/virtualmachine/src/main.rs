
mod bits;
mod binary_components;
mod loader;

fn main() {
    let binary_name = name_of_binary_to_execute();
    let binary_bytes = bytes_from_binary(&binary_name);
    // dbg!(&binary_bytes);
    // let header = loader::BinaryHeader::extract_from_bytes(&binary_bytes[..]);
    // dbg!(header.unwrap());
}

fn name_of_binary_to_execute() -> String {
    std::env::args().collect::<Vec<String>>().into_iter()
    .nth(1).unwrap_or_else(|| {
        eprintln!("error: must provide binary to execute");
        std::process::exit(1)
    })
}

fn bytes_from_binary(path: &String) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|_| {
        eprintln!("error: cannot read binary file: {:?}", path);
        std::process::exit(1)
    })
}
