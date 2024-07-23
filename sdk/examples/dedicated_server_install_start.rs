use ovh_sdk::{ClientDedicatedServerInstallStartExt};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("You should provide a service name as a first argument");
        return;
    }

    let service_name = args[1].clone();

    let client_id = std::env::var("OVH_CLIENT_ID").unwrap();
    let client_secret = std::env::var("OVH_CLIENT_SECRET").unwrap();

    let config = ovh_sdk::config::Config::new_with_credentials(
        "https://eu.api.ovh.com/v1".into(),
        client_id,
        client_secret,
    );
    let context = ovh_sdk::context::Context::new(config).await.unwrap();
    let client = context.client().unwrap();

    let install_details =
        ovh_sdk::types::DedicatedServerInstallDetails::builder().ssh_key_name(String::from("artps"));
    let install_start = ovh_sdk::types::DedicatedServerInstallStart::builder()
        .details(install_details)
        .partition_scheme_name("default")
        .template_name("ubuntu2404-server_64")
        .user_metadata(vec![]);

    let result = client
        .dedicated_server_install_start()
        .service_name(service_name)
        .body(install_start)
        .send()
        .await.map_err(|e| e.to_string());

    println!("{result:#?}");
}
