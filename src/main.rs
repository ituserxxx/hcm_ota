use actix_files::NamedFile;
use actix_web::{App,get,post,web, HttpServer, HttpResponse,http::header::ContentType, Result, HttpRequest,Error};
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
    token: String,
    bin: String,
}

// 示例：curl http://ip:port/api/get_token
#[get("/api/get_token")]
async fn get_token()  -> Result<HttpResponse, Error> {
    let config_data = read_to_string("config.json")?;
    let config: Config = from_str(&config_data)?;
    let data = json!({
        "token": config.token,
    });
    // 返回 message 字段
    Ok(HttpResponse::Ok().content_type(ContentType::json()).body(data.to_string()))
}

// 示例：curl http://ip:port/api/get_bin/xxxx
#[get("/api/get_bin/{token}")]
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



/*
curl --location --request POST "http://172.16.9.103:9998/api/upload" ^
--header "User-Agent: Apifox/1.0.0 (https://apifox.com)" ^
--header "Host: 172.16.9.103:9998" ^
--header "Connection: keep-alive" ^
--form "filename=@\"cmMtdXBsb2FkLTE3MjYyMDcxODM2NTAtMg==/a.yml\""
  */
 
// 上传文件
#[post("/api/upload_bin")]
async fn upload_bin(mut multipart: Multipart) ->Result<HttpResponse, Error>  {
    // 处理 Multipart 请求中的所有文件字段
    while let Some(field) = multipart.next().await {
        let field = field.map_err(|e| {
            eprintln!("Failed to process field: {}", e);
            Error::from(actix_web::error::ErrorInternalServerError(e))
        });
        // 处理每个文件字段
        save_file(field?).await?;
    }

    Ok(HttpResponse::Ok().body("Files uploaded and saved successfully"))
}

#[derive(Deserialize)]
struct FormData {
    token: String,
}


/*
curl --location --request POST 'http://172.16.9.103:9998/api/upload_token' \
--header 'User-Agent: Apifox/1.0.0 (https://apifox.com)' \
--header 'Host: 172.16.9.103:9998' \
--header 'Connection: keep-alive' \
--data-urlencode 'token=adsfasdf'

*/
// 上传token
#[post("/api/upload_token")]
async fn upload_token(form: web::Form<FormData>)->Result<HttpResponse, Error> {
     // todo update config file
     let file_path = "config.json";

     // 1. 读取 JSON 文件内容
     let config_data = read_to_string(file_path)?;
 
     // 2. 解析 JSON 数据为 Config 结构体
     let mut config: Config = from_str(&config_data)?;
 
     // 3. 修改 bin 字段的值
     config.token = form.token.to_string();
 
     // 4. 将更新后的 Config 数据转换为 JSON 字符串
     let updated_config_data = to_string_pretty(&config)?;
 
     // 5. 将更新后的 JSON 数据写回文件
     write(file_path, updated_config_data)?;

    Ok(HttpResponse::Ok().body(format!("token update succ: {}", form.token)))
}
 
 // 保存上传的文件
async fn save_file(mut field: actix_multipart::Field) -> Result<(), Error>  {
    // 获取文件的原始文件名
    let content_disposition = field.content_disposition().clone();
    let file_name = content_disposition
    .get_filename()
    .map(|name| name.to_string())
    .unwrap_or_else(|| "uploaded_file".to_string());

    // -----------
    // let file_name = content_disposition
    //     .get_name()
    //     .unwrap_or("uploaded_file")
    //     .to_string();

    // 生成时间戳
    // let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();

    // 构造新的文件名
    // let new_file_name = format!("{}_{}.txt", file_name, timestamp);
    // -----------


    // 使用原本文件名称
    let new_file_name = format!("{}", file_name);

    // 生成文件的保存路径
    let file_path = PathBuf::from(new_file_name);

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


    // todo update config file
    let file_path = "config.json";

    // 1. 读取 JSON 文件内容
    let config_data = read_to_string(file_path)?;

    // 2. 解析 JSON 数据为 Config 结构体
    let mut config: Config = from_str(&config_data)?;

    // 3. 修改 bin 字段的值
    config.bin = file_name.to_string();

    // 4. 将更新后的 Config 数据转换为 JSON 字符串
    let updated_config_data = to_string_pretty(&config)?;

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
