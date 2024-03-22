use std::{
    env,
    fs::{File, OpenOptions},
    io::{self, stdin, stdout, Read, Write},
    process,
};

fn main() {
    // Run the program and capture the result. If there's an error, the error message will be
    // passed to `pause_terminal` to display before pausing.
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        panic!("Error: Must provide 4 paths as arguments:
[1]: \"path\\to\\Slippi Launcher\\netplay\\Sys\\GameSettings\\GALE01r2.ini\",
[2]: \"path\\to\\GeckoCodes.txt\",
[3]: \"path\\to\\Slippi Launcher\\netplay\\User\\GameSettings\\GALE01.ini\",
[4]: \"path\\to\\GeckoCodeHeaders.txt\"");
    }

    match run(args) {
        Ok(_) => {
            println!("Successfully Re-Inserted Gecko Codes! Press any key to close the window...");
            pause_terminal("");
        }
        Err(e) => {
            let error_message: String =
                format!("Error: {}. Press any key to close the window...", e);
            pause_terminal(&error_message);
            process::exit(1);
        }
    }
}

fn run(args: Vec<String>) -> io::Result<()> {
    // Read the contents of the Gecko codes to append
    let mut gecko_codes_contents: String = String::new();
    
    File::open(args[2].to_owned())?.read_to_string(&mut gecko_codes_contents)?;

    println!("Opened codes for appending...{}", args[2].to_owned());

    // Append the codes to the INI file
    
    let mut code_pool_ini: File = OpenOptions::new()
        .write(true)
        .append(true)
        .create(false)
        .open(args[1].to_owned())?;

    println!("Opened codes pool...{}", args[1].to_owned());

    writeln!(code_pool_ini, "\n{}", gecko_codes_contents)?;

    println!("Wrote codes pool...");

    // Repeat the process for the header file and the second INI path
    let mut gecko_codes_header_contents: String = String::new();
    
    File::open(args[4].to_owned())?.read_to_string(&mut gecko_codes_header_contents)?;

    println!("Opened headers...{}", args[4].to_owned());
    
    let mut code_config_ini: File = OpenOptions::new()
        .write(true)
        .append(true)
        .create(false)
        .open(args[3].to_owned())?;

    println!("Opened config...{}", args[3].to_owned());

    writeln!(code_config_ini, "{}", gecko_codes_header_contents)?;

    println!("Wrote config...");

    println!("All done!");
    Ok(())
}

fn pause_terminal(message: &str) {
    let mut stdout: io::Stdout = stdout();
    write!(stdout, "{}", message).unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
