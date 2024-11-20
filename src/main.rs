fn main() {
    println!(r"
_____________________________________________________________________________________
 ________                __                  _______                       __       
|        \              |  \                |       \                     |  \      
 \$$$$$$$$______    ____| $$  _______       | $$$$$$$\  ______   _______  | $$   __ 
   | $$  /      \  /      $$ /       \      | $$__/ $$ |      \ |       \ | $$  /  \
   | $$ |  $$$$$$\|  $$$$$$$|  $$$$$$$      | $$    $$  \$$$$$$\| $$$$$$$\| $$_/  $$
   | $$ | $$    $$| $$  | $$ \$$    \       | $$$$$$$\ /      $$| $$  | $$| $$   $$ 
   | $$ | $$$$$$$$| $$__| $$ _\$$$$$$\      | $$__/ $$|  $$$$$$$| $$  | $$| $$$$$$\ 
   | $$  \$$     \ \$$    $$|       $$      | $$    $$ \$$    $$| $$  | $$| $$  \$$\
    \$$   \$$$$$$$  \$$$$$$$ \$$$$$$$        \$$$$$$$   \$$$$$$$ \$$   \$$ \$$   \$$
_____________________________________________________________________________________
    ");
    println!("Welcome to the Rusty bank!");

    let mut fname: String = String::new();
    let mut lname: String = String::new();
    let mut passphrase: String = String::new();

    println!("Please enter your first name: ");
    std::io::stdin().read_line(&mut fname).expect("error not found");  

    println!("Please enter your last name: ");
    std::io::stdin().read_line(&mut lname).expect("error not found"); 
    
    println!("Please enter your password: ");
    std::io::stdin().read_line(&mut passphrase).expect("error not found");  
    
}
