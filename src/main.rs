mod cli;

fn test1() {
    println!("test1() executed");
}

fn test2() {
    println!("test2() executed");
}

fn test3() {
    println!("test3() executed");
}

fn test4() {
    let name = cli::cli_prompt::input("Your name: ");
    let age = cli::cli_prompt::input("Your age: ");
    println!("Name: {:?}; Age: {:?}", name, age);
}

fn main() {
    // Change the size of array in cli module, under struct on every array.
    // Sizes must match, or else the module will crash.
    // For example, the size of these arrays is 4.
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
        funcs: [test1, test2, test3, test4], // Don't add "()" to these functions. Just pass them with their names, like this.
    };
    cli_test.clear_terminal();
    cli_test.print_header();

    loop {
        cli_test.cmd_listener(); // Use the 'loop' to listen for commands. Type 'exit' to stop the loop.
    }
}
