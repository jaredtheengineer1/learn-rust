//Structs

//can i create a struct outside of main? 
//i assume yes as i writ this

struct User {
  active: bool,
  username: String,
  email: String
}
// i sure can

fn main() {
  let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("user.email.com")
  };
}