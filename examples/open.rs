use find_editor::Finder;
use std::io::Write;

fn main() {
    let finder = Finder::new();
    let mut f = tempfile::Builder::new()
        .prefix("open-editor-")
        .suffix(".txt")
        .tempfile()
        .expect("Should be able to create a temporary file");
    writeln!(f, "Feel free to edit this file and see your changes!")
        .expect("Should be able to write to the file");
    let (_, filename) = f.keep().expect("Should be able to keep the file");
    let editor = finder.editor_name();
    println!("Calling `{} {}`...", editor, filename.display());
    finder
        .open_editor(&filename, true)
        .expect("Should be able to open the editor");
    println!("Open {} to see your changes.", filename.display());
    println!("It is a good idea to delete the file when you are finished.");
}
