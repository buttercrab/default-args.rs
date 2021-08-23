pub mod config {
    use default_args::default_args;

    // common server configuration
    #[derive(Debug)]
    pub struct ServerConfig {
        host: String,
        http_port: u8,
        https_port: Option<u8>,
        log_level: u8,
        log_path: String,
    }

    // using `default_args!`, you can set default values easily!
    default_args! {
        #[inline]
        export pub fn crate::config::make_config<S1, S2>(
            host: S1 = "0.0.0.0",
            http_port: u8 = 80,
            https_port: Option<u8> = Some(443),
            log_level: u8 = 2,
            log_path: S2 = "./server.log",
        ) -> ServerConfig
        where
            S1: AsRef<str>,
            S2: AsRef<str>,
        {
            ServerConfig {
                host: host.as_ref().to_string(),
                http_port,
                https_port,
                log_level,
                log_path: log_path.as_ref().to_string(),
            }
        }
    }
}

fn main() {
    // you can modify values intuitively
    let config = make_config!("127.0.0.1", https_port = None, log_path = "./log.txt");
    println!("{:?}", config);
}
