use webserver::{database::create_user, database::establish_connection};
use std::io::{stdin, Read};


fn main() {
    let connection = &mut establish_connection();

    let mut email = String::new();
    let mut password = String::new();
    let mut library = String::new();

    println!("Enter your email?");
    stdin().read_line(&mut email).unwrap();
    let email = email.trim_end(); // Remove the trailing newline

    println!(
        "\nOk! Enter the password for {}",
        email, 
    );
    stdin().read_line(&mut password).unwrap();
    let password = password.trim_end(); // Remove the trailing newline

    println!(
        "\nOk! Now enter the first library_id");
    stdin().read_line(&mut library).unwrap();
    let library = library.trim_end(); // Remove the trailing newline


    let user = create_user(connection, &email, &password, &library);
    println!("\nSaved user {} with id {}", user.email, user.id);
}
