# rust-cli-manager
Call functions with their names from live CLI interface, linux terminal style.

## Usage 
Initialize cli_prompt module like this:
```
let cli_test = cli::cli_prompt::cli_prompt::CliPrompt {
      header: "Welcome to my weird CLI manager",
      prefix: "weird.cli>",
      cmds: ["test1", "test2", "test3", "test4"],
      descr: [
          "executes test1",
          "executes test2",
          "executes test3",
          "executes test4",
      ],
      funcs: [test1, test2, test3, test4],
  };
```
Make sure that the sizes of these arrays match, or else the module will crash.
To add more than 4 commands, edit the cli_prompt.rs struct, "CliPrompt". 

```
pub struct CliPrompt<'a> {
    pub header: &'a str,
    pub prefix: &'a str,
    pub cmds: [&'a str; length],
    pub descr: [&'a str; length],
    pub funcs: [fn(); length],
}
    
```
The 'length' value must match the size of commands list because of the limitations of Rust.

## Other commands
```
cli_test.clear_terminal() - clears the terminal by summoning satan with "\x1B[2J\x1B[1;1H" 
cli_test.print_header() - prints the value of the 'header' from struct
cli::cli_prompt::input(user_message) - use this function to get arguments from user. 'user_message' will be displayed on the same line as 'input'.
```

Check out the main.rs file for the usage example. 
