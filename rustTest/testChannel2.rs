use std::sync::mpsc;
use std::thread;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/* Example does not compile
cannot move out of `user1` because it is borrowed
  --> testChannel.rs:28:17
   |
27 |         let mut name = &mut user1.username;
   |                        ------------------- borrow of `user1.username` occurs here
28 |         tx.send(user1).unwrap();
   |                 ^^^^^ move out of `user1` occurs here
29 |         change(name);
   |                ---- borrow later used here

error: aborting due to 2 previous errors; 1 warning emitted

*/
fn main() {

    let (tx, rx) = mpsc::channel(); //create channel

    thread::spawn(move || {

        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        let name = &mut user1.username;
        tx.send(user1).unwrap();
        change(name);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received.username);
}

fn change(some_string: &mut String) {
    some_string.push_str(", notme");
}