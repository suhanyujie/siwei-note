/// 因为需要接收 tower 的回调请求，所以需要启动服务器，接收请求。
/// 要想接收到 tower 发送过来的请求，还需要本地安装一个[内网穿透](https://github.com/open-dingtalk/pierced)
use actix_web::{get, post, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

/// 处理回调
#[post("/tower/get-token")]
async fn get_token() -> impl Responder {
    let url = "https://tower.im/oauth/authorize?client_id=a3d847c065114fefa86421638555f2969e0f9f5377b13b005f98b84f950f9961&redirect_uri=urn:ietf:wg:oauth:2.0:oob&response_type=code";
    let access_token = "122ecd1444eefe3de7e86f87afa0f11b2a5812757b5246fa2c025d3c8e45f006";
    format!("Hello post request...")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8180;

    println!("http server running at port: {}", port);
    HttpServer::new(|| App::new().service(index))
        .bind(format!("{}{}", "127.0.0.1:", port))?
        .run()
        .await
}
