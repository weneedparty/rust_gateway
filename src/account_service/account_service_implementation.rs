use tonic::{transport::Server, Request, Response, Status};

pub mod account_service {
    tonic::include_proto!("account_service"); //This is the package name?
}

use account_service::account_service_server::{AccountService, AccountServiceServer};
use account_service::{
    HelloReply, HelloRequest, JwtIsOkReply, JwtIsOkRequest, JwtObject, RegisterConfirmReply,
    RegisterConfirmRequest, RegisterReply, RegisterRequest,
};

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::account_service::python_account_service;

// #[derive(Debug, Default)]
#[derive(Debug)]
pub struct MyAccountService {
    shared_string: Arc<Mutex<String>>,
    //use Arc<Mutex<T>> to share variables across threads
}

#[tonic::async_trait]
impl AccountService for MyAccountService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = account_service::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }

    async fn user_register_request(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterReply>, Status> {
        println!("Got a request: {:?}", request);

        let mut reply = account_service::RegisterReply {
            result: "OK".to_string(),
            error: "".to_string(),
        };

        let result =
            python_account_service::user_register_request(request.into_inner().email).await;
        match result {
            Ok(success) => {
                println!("register request is sent: {:?}", result);
                if success {
                    reply.result = String::from("OK");
                } else {
                    reply.result = String::from("Not OK");
                }
            }
            Err(err) => {
                println!("Error on pre register request: {:?}", err);
                reply.error = String::from(err.to_string());
            }
        }

        Ok(Response::new(reply))
    }

    async fn user_register_confirm(
        &self,
        request: Request<RegisterConfirmRequest>,
    ) -> Result<Response<RegisterConfirmReply>, Status> {
        println!("Got a request: {:?}", request);

        let mut reply = account_service::RegisterConfirmReply {
            // message: format!("Hello {}!", request.into_inner().name).into(),
            result: Some(JwtObject {
                jwt: "".to_string(),
            }),
            error: "".to_string(),
        };

        let result = python_account_service::user_register_confirm(
            request.get_ref().email.clone(),
            request.get_ref().random_string.clone(),
        )
        .await;

        match result {
            Ok(jwt) => {
                println!("register request confirm is made, we got jwt: {:?}", jwt);
                reply.result = Some(JwtObject {
                    jwt: jwt.to_string(),
                });
            }
            Err(err) => {
                println!("Error on register confirm: {:?}", err);
                reply.error = String::from(err.to_string());
            }
        }

        Ok(Response::new(reply))
    }

    async fn jwt_is_ok(
        &self,
        request: Request<JwtIsOkRequest>,
    ) -> Result<Response<JwtIsOkReply>, Status> {
        println!("Got a request: {:?}", request);

        let mut reply = account_service::JwtIsOkReply {
            ok: false,
            email: "".to_string(),
        };

        let result = python_account_service::auth_jwt(request.get_ref().jwt.clone()).await;

        match result {
            Ok(email) => {
                println!("jwt auth is made, we got email: {:?}", email);
                reply.ok = true;
                reply.email = email.to_string();
            }
            Err(err) => {
                println!("Error on jwt auth: {:?}", err);
                reply.ok = false;
            }
        }

        Ok(Response::new(reply))
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let address_string = "0.0.0.0:40054";
    let addr = address_string.parse()?;

    let a_string = Arc::new(Mutex::new(String::from("Hello, world!")));
    let my_account_service = MyAccountService {
        shared_string: a_string,
    };

    let svc = AccountServiceServer::new(my_account_service);

    println!("Account Server is running on http://{} ...", address_string);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
