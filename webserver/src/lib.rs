use actix::prelude::*;
use actix_files::Files;
use actix_web::{
    get, middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, http::header::ContentType,
};
use actix_web_actors::ws::{self, CloseReason};
use global_lib::{bus::{BusLidarScan, BusRoverMovement, BusIMURotation, BusManager}, types::RoverMovement, CONFIG};
use include_dir::{Dir, include_dir};
use lazy_static::lazy_static;
use log::info;
use smith_core::Smith;
use smith_types::{WebPacket, IMURotationData};
use std::{
    fs,
    time::{Duration, Instant, SystemTime}, sync::{RwLock, Arc}, slice::Iter, ops::Deref, rc::Rc, str::FromStr, io::Write,
};
mod smith_types;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Message,Clone)]
#[rtype(result = "()")]
pub struct BroadcastMessage(Box<[u8]>);
impl Handler<BroadcastMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.binary( msg.0);
    }
}


pub struct WebSocketSystem {
    clients: Vec<Addr<MyWebSocket>>,
}



impl WebSocketSystem {
    pub fn new() -> Self {
        WebSocketSystem { clients: Vec::new() }
    }

    pub fn add_client(&mut self, addr: Addr<MyWebSocket>) {
        self.clients.push(addr);
    }

    pub fn clients(&self)->&[Addr<MyWebSocket>]{
        &self.clients
    }

    pub fn remove_client(&mut self, addr: &Addr<MyWebSocket>) {
        self.clients.retain(|a| a != addr);
    }

    pub fn send_message_to_all(&self, message: &str) {
        for client in &self.clients {
            // client.do_send(BroadcastMessage(message.to_owned()));
            
        }
    }
}

pub struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    addr: Option<Addr<MyWebSocket>>,
    movement_bus: BusRoverMovement
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        self.addr = Some(addr.clone());
        WEB_SOCKETS.write().unwrap().add_client(addr);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        // Remove the address from the global list when a WebSocket connection is closed
        WEB_SOCKETS.write().unwrap().remove_client(self.addr.as_ref().unwrap());
    }
}

impl MyWebSocket {
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                info!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    fn handle_packet(&self, bin: &[u8]) {
        println!("Hi");
        let val: WebPacket = SMITH.binary2rust(bin, &SMITH.get_type("WebPacket").unwrap()).unwrap();
        match val{
            WebPacket::Movement(mov) => {
                let rovmov = match mov{
                    smith_types::WebMovement::Forward => RoverMovement::Forwards,
                    smith_types::WebMovement::Stop => RoverMovement::Stop,
                    smith_types::WebMovement::Left => RoverMovement::Left,
                    smith_types::WebMovement::Right => RoverMovement::Right,
                    smith_types::WebMovement::Speed(s) => RoverMovement::Speed(s),
                };
                println!("Got {rovmov:?}");

                _=self.movement_bus.send(rovmov);
            },
            _=>{}

        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                ctx.text("Not supported");
            }
            Ok(ws::Message::Binary(bin)) => {
                self.handle_packet(&bin);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

lazy_static!{
    static ref SMITH: Smith = Smith::new(&fs::read_to_string("./webserver/web/schema.smith").unwrap());
    static ref WEB_SOCKETS: RwLock<WebSocketSystem> = RwLock::new(WebSocketSystem::new());
}

// static WEB_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/web");

async fn frontendbridge(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {


    ws::start(MyWebSocket { hb: Instant::now(), addr: Default::default(), movement_bus: BusManager::get_rovermovement_bus() }, &req, stream)
}


#[iftree::include_file_tree(
    "
paths = '**'
base_folder = 'web2/public'
"
)]
pub struct Asset {
    relative_path: &'static str,
    // contents_str: &'static str,
    contents_bytes: &'static [u8],
}


pub async fn start_server(lidarbus: BusLidarScan, movementbus: BusRoverMovement, imubus: BusIMURotation) {
    // let index_file = WEB_DIR.get_file("index.html").unwrap();
    // // println!("Found {}",index_file.contents_utf8().unwrap());
    // println!("{:?}",WEB_DIR.files().collect::<Vec<_>>());
    tokio::spawn(async move{
        let mut lidarbus = lidarbus.subscribe();
        let lidar_scan_type = SMITH.get_type("WebPacket").unwrap();
        loop{
            let scan = lidarbus.recv().await.unwrap();
            let smith_lidar_scan = smith_types::WebLidarScan{points: scan.get_points().iter().map(|p|{
                smith_types::WebPoint{x: p.x, y: p.y}
            }).collect(), time: scan.get_time().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis().to_string()};
            let binary = SMITH.rust2binary(&smith_types::WebPacket::LidarScan(smith_lidar_scan), &lidar_scan_type).unwrap();
            let message = BroadcastMessage(binary);
            for ws in WEB_SOCKETS.read().unwrap().clients(){
                ws.do_send(message.clone());
            }
        }   
    });

    tokio::spawn(async move{
        let mut imubus = imubus.subscribe();
        let lidar_scan_type = SMITH.get_type("WebPacket").unwrap();
        loop{
            let scan = imubus.recv().await.unwrap();
            let binary = SMITH.rust2binary(&smith_types::WebPacket::IMURotation(IMURotationData{
                rotation: scan.0,
                time: scan.1.to_string()
            }), &lidar_scan_type).unwrap();
            let message = BroadcastMessage(binary);
            for ws in WEB_SOCKETS.read().unwrap().clients(){
                ws.do_send(message.clone());
            }
        }   
    });


    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        tokio::time::sleep(Duration::from_millis(300)).await;
        std::process::exit(0);
    });


    HttpServer::new(|| {
        let app = App::new()
            .wrap(Logger::new("%a -> %r (%Dms)").log_target("http_log"))
            .service(web::resource("/ws").route(web::get().to(frontendbridge)));
        
            #[cfg(target_os = "macos")]
            let app = app.service(Files::new("/", "./webserver/web2/public").index_file("index.html"));
            #[cfg(not(target_os = "macos"))]
            let app = app.route("/{_:.*}", web::get().to(get_asset));

            app
        // .service(Files::new("/", "../dineryfrontend/dist/").index_file("index.html"))
    })
    .bind(("0.0.0.0", CONFIG.server.port))
    .unwrap()
    .run()
    .await.unwrap();


}

async fn get_asset(path: web::Path<String>) -> impl actix_web::Responder {
    
    let mut path = path.into_inner();
    if path == ""{
        path = "index.html".into()
    }
    
    // For a more efficient lookup, see the `scenario_hash_map` example.
    let mime = mime_guess::from_path(&path);
    let mime = mime.first().unwrap_or(mime::STAR_STAR);
    match ASSETS.iter().position(|asset| asset.relative_path == path) {
        None => actix_web::HttpResponse::NotFound().finish(),
        Some(index) => actix_web::HttpResponse::Ok().content_type(mime).body(ASSETS[index].contents_bytes),
    }
}

// async fn get_embedded_file(path: web::Path<String>) -> impl actix_web::Responder{
//     let path = path.into_inner();

//     WEB_DIR.files()
//     todo!()
// }