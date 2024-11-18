use self::database::models::*;
use diesel::prelude::*;
use webserver::*;

fn main() {
    use self::database::schema::users::dsl::*; // only when single table

    let connection = &mut database::establish_connection();
    let results = users
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.id);
        println!("-----------\n");
        println!("{}", user.email);
        println!("-----------\n");
        println!("{}", user.password);
        println!("-----------\n");
        println!("{}", user.library);
    }
}
