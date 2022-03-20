pub mod room_control_service_implementation {
    use tokio::runtime::Builder as RuntimeBuilder;
    use tonic::{transport::Server, Request, Response, Status};

    const TYPESCRIPT_ROOM_CONTROL_SERVICE_ADDRESS: &str = "http://127.0.0.1:40053/";

    pub mod room_control_service {
        tonic::include_proto!("room_control_service"); //This is the package name?
    }

    use crate::account_service::python_account_service;

    use room_control_service::room_control_service_client::RoomControlServiceClient;
    use room_control_service::room_control_service_server::{
        RoomControlService, RoomControlServiceServer,
    };
    use room_control_service::{
        AllowJoinRequest, AllowJoinResponse, CreateRoomRequest, CreateRoomResponse,
        DeleteRoomRequest, DeleteRoomResponse, HelloReply, HelloRequest, ListRoomsRequest,
        ListRoomsResponse, RoomInfo,
    };

    // #[derive(Debug, Default)]
    #[derive(Debug)]
    pub struct MyRoomControlService {
        // email: Arc<Mutex<String>>,
    //use Arc<Mutex<T>> to share variables across threads
    }

    #[tonic::async_trait]
    impl RoomControlService for MyRoomControlService {
        async fn say_hello(
            &self,
            request: Request<HelloRequest>,
        ) -> Result<Response<HelloReply>, Status> {
            println!("Got a request: {:?}", request);

            let extension = request.extensions().get::<MyExtension>().unwrap();
            println!("extension data from interceptor is: {}", extension.email);

            let reply = HelloReply {
                message: format!("Hello {}!", request.into_inner().name).into(),
            };

            Ok(Response::new(reply))
        }

        async fn create_room(
            &self,
            request: Request<CreateRoomRequest>,
        ) -> Result<Response<CreateRoomResponse>, Status> {
            println!("Got a request: {:?}", request);

            let mut reply = CreateRoomResponse { success: false };

            let client =
                RoomControlServiceClient::connect(TYPESCRIPT_ROOM_CONTROL_SERVICE_ADDRESS).await;
            match client {
                Ok(mut the_client) => {
                    let client_request = tonic::Request::new(CreateRoomRequest {
                        room_name: request.into_inner().room_name,
                    });
                    let response = the_client.create_room(client_request).await?;
                    reply.success = response.into_inner().success;
                }
                Err(_err) => {}
            }

            Ok(Response::new(reply))
        }

        async fn allow_join(
            &self,
            request: Request<AllowJoinRequest>,
        ) -> Result<Response<AllowJoinResponse>, Status> {
            println!("Got a request: {:?}", request);
            let extension = request.extensions().get::<MyExtension>().unwrap();
            let email = (&extension.email).clone().to_string();

            let mut reply = AllowJoinResponse {
                access_token: String::from(""),
            };

            let client =
                RoomControlServiceClient::connect(TYPESCRIPT_ROOM_CONTROL_SERVICE_ADDRESS).await;
            match client {
                Ok(mut the_client) => {
                    let client_request = tonic::Request::new(AllowJoinRequest {
                        room_name: request.get_ref().room_name.clone(),
                        identity: email,
                    });
                    let response = the_client.allow_join(client_request).await?;
                    reply.access_token = response.into_inner().access_token;
                }
                Err(_err) => {}
            }

            Ok(Response::new(reply))
        }

        async fn list_rooms(
            &self,
            request: Request<ListRoomsRequest>,
        ) -> Result<Response<ListRoomsResponse>, Status> {
            println!("Got a request: {:?}", request);

            let mut room_list: Vec<RoomInfo> = Vec::new();
            room_list.push(RoomInfo {
                room_name: String::from(""),
                number_of_participants: 0,
            });
            let mut reply = ListRoomsResponse { rooms: room_list };

            let client =
                RoomControlServiceClient::connect(TYPESCRIPT_ROOM_CONTROL_SERVICE_ADDRESS).await;
            match client {
                Ok(mut the_client) => {
                    let client_request = tonic::Request::new(ListRoomsRequest {});
                    let response = the_client.list_rooms(client_request).await?;
                    reply.rooms = response.into_inner().rooms;
                }
                Err(_err) => {}
            }

            Ok(Response::new(reply))
        }

        async fn delete_room(
            &self,
            request: Request<DeleteRoomRequest>,
        ) -> Result<Response<DeleteRoomResponse>, Status> {
            println!("Got a request: {:?}", request);

            let mut reply = DeleteRoomResponse { success: true };

            let client =
                RoomControlServiceClient::connect(TYPESCRIPT_ROOM_CONTROL_SERVICE_ADDRESS).await;
            match client {
                Ok(mut the_client) => {
                    let client_request = tonic::Request::new(DeleteRoomRequest {
                        room_name: request.into_inner().room_name,
                    });
                    let response = the_client.delete_room(client_request).await?;
                    reply.success = response.into_inner().success;
                }
                Err(_err) => {}
            }

            Ok(Response::new(reply))
        }
    }

    /// This function will get called on each inbound request, if a `Status`
    /// is returned, it will cancel the request and return that status to the
    /// client.
    fn intercept2(mut req: Request<()>) -> Result<Request<()>, Status> {
        println!("Intercepting request: {:?}", req);

        let metadata = req.metadata();
        let jwt_: &str = metadata.get("jwt").unwrap().to_str().unwrap().into();
        let jwt = String::from(jwt_).clone();
        println!("The JWT is: {:?}", jwt);

        let handle = std::thread::spawn(move || {
            let mut returnd_email = String::from("");
            let thread_result = RuntimeBuilder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let result = python_account_service::auth_jwt(jwt.to_string()).await;
                    match result {
                        Ok(temp_email) => {
                            println!("jwt auth is made, we got email: {:?}", temp_email);
                            returnd_email = temp_email.clone();
                            return Ok(temp_email);
                        }
                        Err(err) => {
                            println!("Error on jwt auth: {:?}", err);
                            return Err(Status::new(tonic::Code::Unauthenticated, "Invalid JWT"));
                        }
                    }
                });
            match thread_result {
                Ok(_abc) => {}
                Err(_err) => {}
            }
            return returnd_email;
        });
        let email = handle.join().expect("interceptor thread panicked");

        if email.len() == 0 {
            return Err(Status::new(tonic::Code::Unauthenticated, "Invalid JWT"));
        } else {
            println!("jwt auth is made, we got email: {:?}", email);
            req.extensions_mut().insert(MyExtension {
                // we can do a parse to get the email from the request
                // then sent it to other rpcs in this file
                // see https://github.com/hyperium/tonic/blob/master/examples/src/interceptor/server.rs
                email: email.clone(),
            });
            return Ok(req);
        }
    }

    struct MyExtension {
        email: String,
    }

    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let address_string = "0.0.0.0:40055";
        let addr = address_string.parse()?;

        let my_room_control_service = MyRoomControlService {};

        // See examples/src/interceptor/client.rs for an example of how to create a
        // named interceptor that can be returned from functions or stored in
        // structs.
        let svc = RoomControlServiceServer::with_interceptor(my_room_control_service, intercept2);

        println!(
            "Room Control Server is running on http://{} ...",
            address_string
        );

        Server::builder().add_service(svc).serve(addr).await?;

        Ok(())
    }
}
