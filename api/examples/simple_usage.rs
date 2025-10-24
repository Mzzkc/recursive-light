use api::prompt_engine::FrameworkState;
use api::{prompt_engine, LlmConfig, LlmFactory, VifApi};
use dotenv::dotenv;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut domain_registry = prompt_engine::DomainRegistry::new();
    domain_registry.register_domain(Box::new(api::domains::ComputationalDomain));
    domain_registry.register_domain(Box::new(api::domains::ScientificDomain));
    domain_registry.register_domain(Box::new(api::domains::CulturalDomain));
    domain_registry.register_domain(Box::new(api::domains::ExperientialDomain));

    let framework_state = FrameworkState {
        domain_registry,
        boundaries: vec![
            prompt_engine::BoundaryState {
                name: "CD-SD".to_string(),
                permeability: 0.8,
                status: "Active".to_string(),
            },
            prompt_engine::BoundaryState {
                name: "SD-CuD".to_string(),
                permeability: 0.5,
                status: "Active".to_string(),
            },
        ],
        identity: "User Identity".to_string(),
    };

    let llm_config = LlmConfig {
        api_key: "YOUR_OPENAI_API_KEY".to_string(),
        provider_name: "openai".to_string(),
        model_name: "text-davinci-003".to_string(),
    };
    let provider = LlmFactory::create_llm(&llm_config);
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut vif_api = VifApi::new(provider, framework_state, &database_url)
        .await
        .unwrap();

    let user_input = "Hello, world!";
    let user_id = Uuid::parse_str("936DA01F-9ABD-4D9D-80C7-02AF85C822A8").unwrap();
    match vif_api.process_input(user_input, user_id).await {
        Ok(response) => {
            println!("Response: {}", response);
            match vif_api.get_latest_snapshot(user_id).await {
                Some(latest_snapshot) => println!("Latest Snapshot: {:?}", latest_snapshot),
                None => println!("No latest snapshot available"),
            }
        }
        Err(e) => println!("Error processing input: {}", e),
    }
}
