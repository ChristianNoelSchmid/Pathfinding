use std::io;
use std::io::Write;

///
/// Recieves user input and returns
/// 
pub fn input(same_line: bool) -> String
{
    // Print a input prompt;
    if !same_line 
    {
        print!("  >> "); 
    }
    io::stdout().flush().unwrap();

    // Write input to String and return
    let mut inp = String::new();
    io::stdin().read_line(&mut inp)
        .unwrap();

    // Trim the '\n' off end of String (since
    // read_line writes end-line onto String)
    return format!("{}", inp.trim_end_matches('\n'));
}

///
/// Waits for user to press ENTER before continuing
/// 
pub fn wait_for_enter()
{
    print!("Press ENTER to continue...");
    let _ = input(true);
}

///
/// Clears the console screen. Waits for
/// terminal to call command before returning, so
/// sequential lines are deleted in the lag
/// 
pub fn clear_screen()
{
    std::process::Command::new("clear").spawn()
        .expect("Failed to clear screen!")
        .wait()
        .expect("Failed to sleep program!");
}