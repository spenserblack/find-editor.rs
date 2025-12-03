# find-editor.rs

Finds and opens an editor to edit a file. Useful if you want to make it easy for your
users to edit config files.

## Usage

You can find runnable examples in [`examples/`](./examples).

```rust
use find_editor::Finder;

let finder = Finder::with_extra_environment_variables(["MY_TOOL_EDITOR"]);

// Get an editor value
let editor = finder.editor_name();
println!("The raw editor command is: {editor}");

// Parse the editor string and assert it's on $PATH
let (editor_command, editor_args) = finder.which_editor().unwrap();
println!("The editor command is {editor_command:?} with the arguments {editor_args:?}");

// Open an editor on a file
finder.open_editor("config.toml", true).unwrap();
```

### Optional features

- `split`: Sometimes editors are not only a command name, but also a list of arguments
  to pass to the command. `code --wait` is a common example. This feature provides
  `split_editor_name`, which helps split an editor's text into the command and any
  arguments.
- `which`: This provides `which_editor`, which will split the editor (see feature
  `split`), and then find the command on `$PATH`. This helps assert that the command
  is callable. Also, Windows will run executables in the current directory when running
  a command. `which_editor` helps prevent that possible security issue by *only* finding
  executables on `$PATH`.
- `open`: This provides the `open_editor` function. `which_editor` (see feature `which`)
  and `split_editor_name` are both used to ensure that the editor is safely executed.
