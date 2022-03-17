pub mod account_service_implementation {
    use tonic::{transport::Server, Request, Response, Status};

    pub mod account_service {
        tonic::include_proto!("account_service"); //This is the package name?
    }

    use account_service::account_service_server::{AccountService, AccountServiceServer};
    use account_service::{
        Empty, HelloReply, HelloRequest, JwtIsOkReply, JwtIsOkRequest, JwtObject,
        RegisterConfirmReply, RegisterConfirmRequest, RegisterReply, RegisterRequest,
    };

    use std::sync::Arc;
    use tokio::sync::Mutex;

    use futures::stream::StreamExt;
    use std::pin::Pin;
    use tokio::sync::broadcast;
    use tokio_stream::wrappers::BroadcastStream;
    use tokio_stream::Stream;

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

            let extension = request.extensions().get::<MyExtension>().unwrap();
            println!("extension data from interceptor is: {}", extension.jwt);

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

            let reply = account_service::RegisterReply {
                // message: format!("Hello {}!", request.into_inner().name).into(),
                result: "OK".to_string(),
                error: "".to_string(),
            };

            Ok(Response::new(reply))
        }

        async fn user_register_confirm(
            &self,
            request: Request<RegisterConfirmRequest>,
        ) -> Result<Response<RegisterConfirmReply>, Status> {
            println!("Got a request: {:?}", request);

            let reply = account_service::RegisterConfirmReply {
                // message: format!("Hello {}!", request.into_inner().name).into(),
                result: Some(JwtObject {
                    jwt: "".to_string(),
                }),
                error: "".to_string(),
            };

            Ok(Response::new(reply))
        }

        async fn jwt_is_ok(
            &self,
            request: Request<JwtIsOkRequest>,
        ) -> Result<Response<JwtIsOkReply>, Status> {
            println!("Got a request: {:?}", request);

            let reply = account_service::JwtIsOkReply { ok: true };

            Ok(Response::new(reply))
        }
    }

    /// This function will get called on each inbound request, if a `Status`
    /// is returned, it will cancel the request and return that status to the
    /// client.
    fn intercept(mut req: Request<()>) -> Result<Request<()>, Status> {
        println!("Intercepting request: {:?}", req);

        let metadata = req.metadata();
        let jwt: &str = metadata.get("jwt").unwrap().to_str().unwrap().into();
        println!("The JWT is: {:?}", jwt);

        // Set an extension that can be retrieved by `say_hello`
        req.extensions_mut().insert(MyExtension {
            // we can do a parse to get the email from the request
            // then sent it to other rpcs in this file
            // see https://github.com/hyperium/tonic/blob/master/examples/src/interceptor/server.rs
            jwt: "foo".to_string(),
        });

        // Ok(req)
        return Ok(req);
        // return Err(Status::invalid_argument("jwt is invalid"));
    }

    struct MyExtension {
        jwt: String,
    }

    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let address_string = "0.0.0.0:40052";
        let addr = address_string.parse()?;

        let a_string = Arc::new(Mutex::new(String::from("Hello, world!")));
        let my_account_service = MyAccountService {
            shared_string: a_string,
        };

        // See examples/src/interceptor/client.rs for an example of how to create a
        // named interceptor that can be returned from functions or stored in
        // structs.
        let svc = AccountServiceServer::with_interceptor(my_account_service, intercept);

        println!("Account Server is running on http://{} ...", address_string);

        Server::builder().add_service(svc).serve(addr).await?;

        Ok(())
    }

    // #[tokio::main]
    // pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //     let address_string = "0.0.0.0:40053";
    //     let addr = address_string.parse()?;

    //     let a_string = Arc::new(Mutex::new(String::from("Hello, world!")));
    //     let my_account_service = MyAccountService {
    //         shared_string: a_string,
    //     };

    //     // See examples/src/interceptor/client.rs for an example of how to create a
    //     // named interceptor that can be returned from functions or stored in
    //     // structs.
    //     let svc = AccountServiceServer::with_interceptor(my_account_service, intercept);

    //     println!("Account Server is running on http://{} ...", address_string);

    //     Server::builder().add_service(svc).serve(addr).await?;

    //     Ok(())
    // }
}
