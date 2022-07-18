pub mod cli_prompt {
    pub struct CliPrompt<'a> {
        pub header: &'a str,
        pub prefix: &'a str,
        pub cmds: [&'a str; 4],
        pub descr: [&'a str; 4],
        pub funcs: [fn(); 4],
    }

    impl CliPrompt<'_> {
        fn input(user_message: &str) -> std::io::Result<String> {
            print!("{}", user_message);
            // buffer has to be flushed
            std::io::Write::flush(&mut std::io::stdout())?;
            let mut buffer: String = String::new();
            std::io::stdin().read_line(&mut buffer)?;
            Ok(buffer.trim().to_owned().to_string())
        }

        fn man_generator(&self) {
            println!();
            for i in 0..self.cmds.len() {
                println!("{} - {}", self.cmds[i], self.descr[i]);
            }
            println!();
        }

        pub fn print_header(&self) {
            println!("{}", self.header);
            println!("Type 'help' to get the command list ");
            println!();
        }

        fn execute_cmd(&self, cmd: &str) {
            if cmd == "clear" {
                self.clear_terminal();
            } else if cmd == "help" {
                self.man_generator();
            } else if cmd == "exit" {
                println!("Goodbye...");
                std::process::exit(1);

            } else if self.cmds.contains(&cmd) {
                let index = self.cmds.iter().position(|x| x == &cmd).unwrap();
                self.funcs[index]();
            } else {
                println!("Command '{}' doesn't exist. Executing 'help'... ", cmd);
                self.man_generator();
            }
        }

        pub fn cmd_listener(&self) {
            let input_string = &CliPrompt::<'_>::input(&format!("{} ", self.prefix)).unwrap();
            let mut command_chunks: Vec<&str> = input_string.split_whitespace().collect();
            let cmd = command_chunks[0];

            if command_chunks.len() > 0 {
                command_chunks.remove(0);
            }

            if command_chunks.len() > 0 {
                self.execute_cmd(cmd)
            } else {
                self.execute_cmd(input_string)
            }
        }
        pub fn clear_terminal(&self) {
            print!("\x1B[2J\x1B[1;1H");
        }
    }
}

pub fn input(user_message: &str) -> std::io::Result<String> {
    print!("{}", user_message);
    // buffer has to be flushed
    std::io::Write::flush(&mut std::io::stdout())?;
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned().to_string())
}
