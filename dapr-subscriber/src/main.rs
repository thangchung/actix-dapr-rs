use actix_web::middleware::Logger;
use actix_web::{get, App, HttpServer, Responder};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use tonic::{transport::Server, Request, Response, Status};

use dapr::{
    appcallback::*,
    dapr::dapr::proto::runtime::v1::app_callback_server::{AppCallback, AppCallbackServer},
};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct AppCallbackService {}

#[tonic::async_trait]
impl AppCallback for AppCallbackService {
    /// Invokes service method with InvokeRequest.
    async fn on_invoke(
        &self,
        _request: Request<InvokeRequest>,
    ) -> Result<Response<InvokeResponse>, Status> {
        Ok(Response::new(InvokeResponse::default()))
    }

    /// Lists all topics subscribed by this app.
    ///
    /// NOTE: Dapr runtime will call this method to get
    /// the list of topics the app wants to subscribe to.
    /// In this example, the app is subscribing to topic `A`.
    async fn list_topic_subscriptions(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ListTopicSubscriptionsResponse>, Status> {
        let topic = "A".to_string();
        let pubsub_name = "pubsub".to_string();

        let list_subscriptions = ListTopicSubscriptionsResponse::topic(pubsub_name, topic);

        Ok(Response::new(list_subscriptions))
    }

    /// Subscribes events from Pubsub.
    async fn on_topic_event(
        &self,
        request: Request<TopicEventRequest>,
    ) -> Result<Response<TopicEventResponse>, Status> {
        let r = request.into_inner();
        let data = &r.data;
        let data_content_type = &r.data_content_type;

        let message = String::from_utf8_lossy(&data);
        println!("Message: {}", &message);
        println!("Content-Type: {}", &data_content_type);

        Ok(Response::new(TopicEventResponse::default()))
    }

    /// Lists all input bindings subscribed by this app.
    async fn list_input_bindings(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ListInputBindingsResponse>, Status> {
        Ok(Response::new(ListInputBindingsResponse::default()))
    }

    /// Listens events from the input bindings.
    async fn on_binding_event(
        &self,
        _request: Request<BindingEventRequest>,
    ) -> Result<Response<BindingEventResponse>, Status> {
        Ok(Response::new(BindingEventResponse::default()))
    }
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> std::result::Result<Response<HelloReply>, Status> {
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[get("/index.html")]
async fn index() -> impl Responder {
    format!("Hello {}!", String::from("abc"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    tokio::spawn(async move {
        let port = 50010;
        let addr: std::net::SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
        let greeter = MyGreeter::default();

        let out_dir_env = std::env!("OUT_DIR");
        println!("path: {:?}", out_dir_env);

        let callback_service = AppCallbackService::default();

        println!("AppCallback server listening on: {}", addr);

        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .add_service(AppCallbackServer::new(callback_service))
            .serve(addr)
            .await
            .expect("Some error message");
    });

    HttpServer::new(|| App::new().wrap(Logger::default()).service(index))
        .bind("0.0.0.0:8088")?
        .run()
        .await
}
