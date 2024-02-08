// App.rs
// Michael Steer: 2024
// 
// Don't delete above whitespace

use tokio;      // Multithreading

mod splash;
mod app;

use splash::ascii_splash::print_splash;
use app::Application;

// Main function
#[tokio::main]
async fn main() {
    println!("Hello world fuck u");
    print_splash();

    let mut holy_moly = Application::new(
        String::from("HOLY MOLY"),
        String::from("V1.0"));

    holy_moly.print_version();
    holy_moly.start();
    
    // ...
    holy_moly.apploop();


    holy_moly.stop();
}
