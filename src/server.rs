use std::env;

#[derive(Debug, Clone)]
pub struct EnvironmentVariables {
    accountservice_NETWORK_NAME: String,
    roommanageservice_NETWORK_NAME: String,
}

mod utils;

mod helloworld_service;
use helloworld_service::helloworld_service_implementation;

mod account_service;
use account_service::account_service_implementation;

mod room_control_service;
use room_control_service::room_control_service_implementation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //   - accountservice_NETWORK_NAME=weloveparty_accountservice
    //   - roommanageservice_NETWORK_NAME=weloveparty_roommanageservice

    let mut environment_variables: EnvironmentVariables = EnvironmentVariables {
        accountservice_NETWORK_NAME: String::from("http://0.0.0.0:40052/"),
        roommanageservice_NETWORK_NAME: String::from("http://0.0.0.0:40053/"),
    };

    match env::var_os("accountservice_NETWORK_NAME") {
        Some(v) => {
            environment_variables.accountservice_NETWORK_NAME =
                String::from("http://") + v.to_str().unwrap() + ":40052/";
        }
        None => println!("$accountservice_NETWORK_NAME is not set"),
    };

    match env::var_os("roommanageservice_NETWORK_NAME") {
        Some(v) => {
            environment_variables.roommanageservice_NETWORK_NAME =
                String::from("http://") + v.to_str().unwrap() + ":40053/";
        }
        None => println!("$roommanageservice_NETWORK_NAME is not set"),
    };

    let _res = tokio::try_join!(
        helloworld_service_implementation::run(),
        account_service_implementation::run(environment_variables.clone()),
        room_control_service_implementation::run(environment_variables.clone())
    );

    // match _res {
    //     Ok((_first, _second, _third)) => {
    //         // do something with the values
    //         println!("processing finished");
    //     }
    //     Err(err) => {
    //         println!("processing failed; error = {}", err);
    //     }
    // }

    Ok(())
}
