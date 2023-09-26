use std::fmt::format;
use std::fs;

#[tokio::main]
async fn main() {

    let files = vec![
        "Arabic",
        "Chinese",
        "Czech",
        "English",
        "French",
        "German",
        "Greece",
        "Hebrew",
        "Italian",
        "Spain",
        "Swedish",
        "Turkish",
        "Ukraine",
        "Zulu"
    ];
    let ext = "txt";
    let path = "/home/gleb/Language";
    for file in files {
        let filename = format!("{}/{}.{}", path, file, ext);
        let data = fs::read_to_string(filename).expect("Unable to find file");
        println!("{data}");
    }

}
