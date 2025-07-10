use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use accounts_manager::AccountsManagerServer;

pub struct BackendLogic {
    runtime: Runtime,
    runtime_thread: JoinHandle<()>
}

impl BackendLogic {
    pub fn run() -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let runtime_thread = runtime.spawn(async {
            let server = AccountsManagerServer::run().await.unwrap();
            // let mut account_connection = AccountConnection::new(&server.get_address().to_string()).unwrap();
            loop {
                // Update logic
                // account_connection.update().await;
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });
        
        Self { runtime, runtime_thread }
    }
}




/////////////////

// 
// use std::time::Duration;
// use accounts_manager::client::{AccountsManagerClient, AccountsManagerResult};
// 
// pub struct AccountConnection {
//     pub client: AccountsManagerClient,
//     pub state: ClientState,
// }
// 
// pub enum ClientState {
//     ServerYetUnknown, // Initial and fallback of fails
//     ServerOff, // Fatal if YetUnknown turns to be off
//     ServerOk, //todo add sub-states
// }
// 
// impl AccountConnection {
//     pub fn new(address: &str) -> AccountsManagerResult<Self> {
//         Ok(Self {
//             client: AccountsManagerClient::new(address)?,
//             state: ClientState::ServerYetUnknown,
//         })
//     }
// 
//     pub async fn update(&mut self) {
//         match self.state {
//             ClientState::ServerYetUnknown => {
//                 match self.client.get_server_status().await {
//                     Ok(server_status) => {
//                         tracing::info!("Server Ok! Motd: '{}'. Accounts count: {}",
//                             server_status.motd,
//                             server_status.accounts_count);
//                         self.state = ClientState::ServerOk;
//                     },
//                     Err(e) => {
//                         tracing::error!("Server check problem: {e}");
//                         self.state = ClientState::ServerOff;
//                     }
//                 }
//             },
//             ClientState::ServerOff => {
//                 tracing::warn!("ServerOff please shut");
//                 tokio::time::sleep(Duration::from_secs(1)).await;
//             },
//             ClientState::ServerOk => {
// 
//             }
//         }
//     }
// }

