use actix_files::NamedFile;
use actix_web::{App,get, HttpServer, HttpResponse,http::header::ContentType, Result, HttpRequest,Error};
use actix_web::http::header::ContentDisposition;
use std::path::PathBuf;
use serde::Deserialize;
use serde_json::from_str;
use std::fs::read_to_string;
use serde_json::json;


// use chrono::Utc; // 需要添加 chrono 依赖
// use actix_web::http::header::ContentDisposition;
// 定义 JSON 文件的结构体
#[derive(Deserialize)]
struct Config {
    token: String,
    bin: String,
}
// 示例：curl http://ip:port/get_token
#[get("/get_token")]
async fn get_token()  -> Result<HttpResponse, Error> {
    let config_data = read_to_string("config.json")?;
    let config: Config = from_str(&config_data)?;
    let data = json!({
        "token": config.token,
    });
    // 返回 message 字段
    Ok(HttpResponse::Ok() .content_type(ContentType::json()).body(data.to_string()))
}
// 示例：curl http://ip:port/get_bin/xxxx
#[get("/get_bin/{token}")]
async fn get_bin(req: HttpRequest) -> Result<HttpResponse> {
    // 获取token路径参数
    let token_param: String = req.match_info().get("token").unwrap_or_default().to_string();

    let config_data = read_to_string("config.json")?;
    let config: Config = from_str(&config_data)?;

    // 检查token是否匹配
    if token_param != config.token {
        return Ok(HttpResponse::Forbidden().finish());
    }
    // 读取并返回文件
    let file_path = PathBuf::from( config.bin.to_string());
    let file = NamedFile::open(file_path)?;

    Ok(file
        .set_content_disposition(ContentDisposition::attachment(config.bin.to_string()))
        .into_response(&req))
}


// async fn save_file(field: actix_multipart::Field) -> Result<HttpResponse> {
//     // 获取文件的原始文件名
//     let content_disposition = field.content_disposition().clone();
//     let file_name = content_disposition
//         .get_filename()
//         .unwrap_or("uploaded_file")
//         .to_string();

//     // 生成时间戳
//     let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();

//     // 构造新的文件名
//     let new_file_name = format!("{}_{}.txt", file_name, timestamp);

//     // 生成文件的保存路径
//     let file_path = PathBuf::from(new_file_name);

//     // 打开文件以进行写入
//     let mut file = File::create(&file_path).map_err(|e| {
//         eprintln!("Failed to create file: {}", e);
//         HttpResponse::InternalServerError().finish()
//     })?;
    
//     // 将接收到的文件内容写入到文件
//     while let Some(Ok(bytes)) = field.next().await {
//         file.write_all(&bytes).map_err(|e| {
//             eprintln!("Failed to write to file: {}", e);
//             HttpResponse::InternalServerError().finish()
//         })?;
//     }

//     Ok(HttpResponse::Ok().body(format!("File uploaded and saved as {}", new_file_name)))
// }
// #[post("/upload")]
// async fn upload_file(multipart: Multipart) -> Result<HttpResponse> {
//     // 处理 Multipart 请求中的所有文件字段
//     while let Some(field) = multipart.next().await {
//         let field = field.map_err(|e| {
//             eprintln!("Failed to process field: {}", e);
//             HttpResponse::InternalServerError().finish()
//         })?;
//         // 处理每个文件字段
//         save_file(field).await?;
//     }

//     Ok(HttpResponse::Ok().body("Files uploaded and saved successfully"))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_token)
            .service(get_bin)
            // .service(upload_file)

    })
    .bind("0.0.0.0:9998")?
    .run()
    .await
}
