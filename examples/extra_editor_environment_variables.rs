use find_editor::Finder;

fn main() {
    const EXTRA_KEY: &str = "FIND_EDITOR_EXAMPLE_EDITOR";
    let finder = Finder::with_extra_environment_variables([EXTRA_KEY]);

    let editor = finder.editor_name();
    println!("Editor: {editor}");
    println!("NOTE: Define ${EXTRA_KEY} and run this example again.");
}
