use solana_client::rpc_client::RpcClient;

pub mod models;



fn main() {
    println!("===========================================");
    println!("          SOLANA INDEXER                   ");
    println!("===========================================");


    let rpc_url = "https://api.devnet.solana.com";
    println!("[*] connecting to RPC endpoint: {}", rpc_url);

    let rpc_client = RpcClient::new(rpc_url.to_string());

    match rpc_client.get_version() {
        Ok(v) => {
            println!("[+] Connected! Node version: {} ", v.solana_core);
        }
        Err(e) => {
            println!("[-] Connection Failed: {} ", e);
            std::process::exit(1);
        }
    }

}

