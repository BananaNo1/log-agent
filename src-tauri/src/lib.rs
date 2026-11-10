use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpStream},
    time::Duration,
};

#[tauri::command]
fn backend_health() -> Result<String, String> {
    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, 18765));

    let mut stream = TcpStream::connect_timeout(
        &address,
        Duration::from_secs(2),
    )
    .map_err(|_| "无法连接 Java 后端，请先启动服务".to_string())?;

    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .map_err(|error| error.to_string())?;

    stream
        .write_all(
            b"GET /health HTTP/1.0\r\n\
              Host: 127.0.0.1\r\n\
              Connection: close\r\n\r\n",
        )
        .map_err(|error| error.to_string())?;

    let mut response = String::new();

    // 只读取有限字节，避免健康检查无限读取。
    stream
        .take(4096)
        .read_to_string(&mut response)
        .map_err(|error| error.to_string())?;

    let status_line = response.lines().next().unwrap_or("");

    if status_line.contains(" 200 ") {
        Ok("Java 后端运行中".to_string())
    } else {
        Err("后端返回了非成功状态".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![backend_health])
        .run(tauri::generate_context!())
        .expect("启动桌面程序失败");
}