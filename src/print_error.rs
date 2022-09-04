use std::io;
use std::io::ErrorKind;

// paging sometimes causes errors wrt to broken pipes, handle it here
pub fn pe(res: Result<(), io::Error>) {
    match res {
        Ok(_) => {}
        Err(e) => {
            match e.kind() {
                ErrorKind::BrokenPipe => {
                    std::process::exit(1); // Don't panick and exit cause it's not our problem
                }
                _ => {
                    println!("Error: {}", e); // print error and exit
                    std::process::exit(crate::ERROR_EXIT_CODE);
                }
            }
        }
    }
}
