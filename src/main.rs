use actix_files::NamedFile;
use actix_web::{App,get,post,web, HttpServer, HttpResponse,http::header::ContentType, Result, HttpRequest,Error,error};
use actix_web::http::header::ContentDisposition;
use std::path::PathBuf;
use serde_json::{to_string_pretty,from_str,json};
use std::fs::{read_to_string, write};
use actix_multipart::Multipart;
use std::fs::File;
use futures::StreamExt;
use chrono::Utc;
use std::io::Write;
use serde::{Deserialize, Serialize};



// 定义 JSON 文件的结构体
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    id: u32,
    token: String,
    bin: String,
}

// 示例：curl http://172.16.9.103:9998/api/get_token/1
#[get("/api/get_token/{id}")]
async fn get_token(path: web::Path<(u32,)>)  -> Result<HttpResponse, Error> {
    let id : u32 = path.into_inner().0;

    let config_data = read_to_string("config.json")?;
    let items: Vec<Config> = serde_json::from_str(&config_data)?;
    let mut data = json!({
        "token": "",
    });
    for v in items {
        if v.id == id{
            data["token"] = v.token.into();
            break;
        }
    }
    Ok(HttpResponse::Ok().content_type(ContentType::json()).body(data.to_string()))
}

// 示例：curl http://ip:port/api/get_bin/xxxx
#[get("/api/get_bin/{token}")]
async fn get_bin(path: web::Path<(String,)>, req: HttpRequest) -> Result<HttpResponse> {
    // 获取token路径参数
    let token_param: String = path.into_inner().0;

    let config_data = read_to_string("config.json")?;
    let items: Vec<Config> = serde_json::from_str(&config_data)?;

    for v in items {
        if v.token == token_param{
            let file_path = PathBuf::from( v.bin.to_string());
            let file = NamedFile::open(file_path)?;
           return Ok(file.set_content_disposition(ContentDisposition::attachment(v.bin.to_string())).into_response(&req));
        }
    }
    return Err(error::ErrorBadRequest("token error"));
}

// 上传token
#[derive(Deserialize)]
struct FormData {
    token: String,
    id: u32,
}
#[post("/api/upload_token")]
async fn upload_token(form: web::Form<FormData>)->Result<HttpResponse, Error> {
     // todo update config file
     let file_path = "config.json";
     let config_data = read_to_string("config.json")?;
     let mut items: Vec<Config> = serde_json::from_str(&config_data)?;

     for (index, v) in items.iter_mut().enumerate() {
        if v.id == form.id{
            items[index].token = form.token.to_string();
            break;
        }
    }
     // 4. 将更新后的 Config 数据转换为 JSON 字符串
     let updated_config_data = to_string_pretty(&items)?;
     // 5. 将更新后的 JSON 数据写回文件
     write(file_path, updated_config_data)?;
    Ok(HttpResponse::Ok().body(format!("token update succ: {}", form.token)))
}
 


/*
curl --location --request POST "http://172.16.9.103:9998/api/upload" ^
--header "User-Agent: Apifox/1.0.0 (https://apifox.com)" ^
--header "Host: 172.16.9.103:9998" ^
--header "Connection: keep-alive" ^
--form "filename=@\"cmMtdXBsb2FkLTE3MjYyMDcxODM2NTAtMg==/a.yml\""
  */
 
// 上传文件
#[post("/api/upload_bin/{id}")]
async fn upload_bin(path: web::Path<(u32,)>,mut multipart: Multipart) ->Result<HttpResponse, Error>  {
    let id : u32 = path.into_inner().0;
 
    // 处理 Multipart 请求中的所有文件字段
    while let Some(field) = multipart.next().await {
        let field = field.map_err(|e| {
            eprintln!("Failed to process field: {}", e);
            Error::from(actix_web::error::ErrorInternalServerError(e))
        });
        // 处理每个文件字段
        save_file(field?,id).await?;
    }

    Ok(HttpResponse::Ok().body("Files uploaded and saved successfully"))
}

 // 保存上传文件
async fn save_file(mut field: actix_multipart::Field,id: u32) -> Result<(), Error>  {
    // 获取文件的原始文件名
    let content_disposition = field.content_disposition().clone();
    let file_name = content_disposition
    .get_filename()
    .map(|name| name.to_string())
    .unwrap_or_else(|| "uploaded_file".to_string());

    /* -----------
    // let file_name = content_disposition
    //     .get_name()
    //     .unwrap_or("uploaded_file")
    //     .to_string();

    // 生成时间戳
    // let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();

    // 构造新的文件名
    // let new_file_name = format!("{}_{}.txt", file_name, timestamp);
    // -----------
    */ 

    // 使用原本文件名称
    let new_file_name = format!("{}", file_name);

    // 生成文件的保存路径
    let file_path = PathBuf::from(new_file_name.clone());

    // 打开文件以进行写入
    let mut file = File::create(&file_path).map_err(|e| {
        eprintln!("Failed to create file: {}", e);
        Error::from(actix_web::error::ErrorInternalServerError(e))
    })?;
    
    // 将接收到的文件内容写入到文件
    while let Some(Ok(bytes)) = field.next().await {
        file.write_all(&bytes).map_err(|e| {
            eprintln!("Failed to write to file: {}", e);
            Error::from(actix_web::error::ErrorInternalServerError(e))
        })?;
    }

    let file_path = "config.json";
    let config_data = read_to_string("config.json")?;
    let mut items: Vec<Config> = serde_json::from_str(&config_data)?;

    for (index, v) in items.iter_mut().enumerate() {
       if v.id == id{
           items[index].bin = new_file_name.to_string();
           break;
       }
   }
    // 4. 将更新后的 Config 数据转换为 JSON 字符串
    let updated_config_data = to_string_pretty(&items)?;
    // 5. 将更新后的 JSON 数据写回文件
    write(file_path, updated_config_data)?;
   
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_token)
            .service(get_bin)
            .service(upload_bin)
            .service(upload_token)

    })
    .bind("0.0.0.0:9998")?
    .run()
    .await
}
