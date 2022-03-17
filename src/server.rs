mod utils;

mod helloworld_service;
use helloworld_service::helloworld_service_implementation;

mod account_service;
use account_service::account_service_implementation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("");
    let res = tokio::try_join!(
        helloworld_service_implementation::run(),
        account_service_implementation::run()
    );

    match res {
        Ok((_first, _second)) => {
            // do something with the values
            println!("processing finished");
        }
        Err(err) => {
            println!("processing failed; error = {}", err);
        }
    }

    Ok(())
}
