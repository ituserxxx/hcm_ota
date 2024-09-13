# hcm_ota


rust actix-web 实现上传文件、下载文件

curl --location --request POST "https://somefiles.vtian.top/api/upload_bin" ^
--header "Accept: */*" ^
--header "Host: somefiles.vtian.top" ^
--header "Connection: keep-alive" ^
--form "filename=@\"cmMtdXBsb2FkLTE3MjYyMDcxODM2NTAtMTE=/1.py\""