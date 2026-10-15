/* Program description:
fs => File system
std::fs::File -> is for creating files.
std::fs:fs    -> is for creating folders and deleting files and folders.
*/

use std::fs::File;
use std::fs;
use std::process::Command;

fn main() {
    // Files
      // use std::fs::File;
    File::create("test.txt").unwrap();
    println!("Created file 'test.txt' successfully!.");
    Command::new("ls")
        .arg("-l")
        .arg("test.txt")
        .status()
        .unwrap();
      // use std::fs;
    fs::remove_file("test.txt").unwrap();
    println!("Deleted file 'test.txt' successfully!.");
    Command::new("ls")
        .arg("-l")
        .arg("test.txt")
        .status()
        .unwrap();
    // Folders
      // use std::fs;
    fs::create_dir("TEST").unwrap();
    println!("Created folder 'TEST' successfully!.");
    Command::new("ls")
        .arg("-dl")
        .arg("TEST")
        .status()
        .unwrap();
      // use std::fs;
    fs::remove_dir("TEST").unwrap();
    println!("Deleted folder 'TEST' successfully!.");
    Command::new("ls")
        .arg("-dl")
        .arg("TEST")
        .status()
        .unwrap();
}

/* Example:
file:     test.txt
folder:   TEST
*/

/* Output:
Created file 'test.txt' successfully!.
-rw-r--r-- 1 null staff 0 Jun 21 22:32 test.txt
Deleted file 'test.txt' successfully!.
ls: cannot access 'test.txt': No such file or directory
Created folder 'TEST' successfully!.
drwxr-xr-x 2 null staff 4096 Jun 21 22:32 TEST
Deleted folder 'TEST' successfully!.
ls: cannot access 'TEST': No such file or directory
*/