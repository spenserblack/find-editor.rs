use find_editor::Finder;

fn main() {
    let finder = Finder::new();
    let editor = finder.editor_name();
    println!("The raw editor command is: {editor}");

    let (editor_command, editor_args) = match finder.which_editor() {
        Ok(tuple) => tuple,
        Err(e) => panic!("Failed to parse and find editor: {e}"),
    };

    println!("The editor command is {editor_command:?} with the arguments {editor_args:?}");
}
