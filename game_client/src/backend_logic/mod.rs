use std::time::Duration;
use accounts_manager::AccountsManagerServer;
use accounts_manager::client::{AccountsManagerClient, AccountsManagerClientError, AccountsManagerClientResult};
use accounts_manager::responses::AccountsServerStatus;
use crate::gui::RegisterData;

enum BackendRequest {
    GetServerStatus {
        response: std::sync::mpsc::Sender<AccountsManagerClientResult<AccountsServerStatus>>,
    },
    RegisterNewAccount {
        register_data: RegisterData,
        response: std::sync::mpsc::Sender<AccountsManagerClientResult<()>>,
    }
}

#[derive(Debug)]
pub struct BackendLogic {
    sender: std::sync::mpsc::Sender<BackendRequest>,
    runtime_thread: std::thread::JoinHandle<()>,
}

impl BackendLogic {
    pub fn new() -> Self {
        tracing::debug!("new...");

        let (tx, rx) = std::sync::mpsc::channel::<BackendRequest>();
        let runtime_thread = std::thread::spawn(move || {
            tracing::debug!("Thread started!");
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                tracing::debug!("Async task started!");

                println!("Starting test servers!");

                // Comment to test with server offline
                let account_manager_server = AccountsManagerServer::run().await.unwrap();
                let account_manager_server_address = account_manager_server.get_address();
                let account_manager_server_address = account_manager_server_address.to_string();

                // Uncomment to test with server offline
                // let account_manager_server_address = "0.0.0.0:1234".to_owned();

                // Note: client do not need running server at creation point
                let account_manager_client =  AccountsManagerClient::new(account_manager_server_address.as_str()).unwrap();

                loop {
                    if let Ok(req) = rx.try_recv() {
                        match req {
                            BackendRequest::GetServerStatus { response } => {
                                let status = account_manager_client.get_server_status().await;
                                let _ = response.send(status);
                            },
                            BackendRequest::RegisterNewAccount { register_data, response } => {
                                let status = account_manager_client.request_create_account(register_data.username, register_data.password).await;
                                let _ = response.send(status);
                            }
                        }
                    }

                    tokio::time::sleep(Duration::from_millis(500)).await;
                    tracing::trace!("Tick");
                }
            });
        });

        Self {
            sender: tx,
            runtime_thread,
        }
    }

    pub fn fetch_server_status(&self) -> AccountsManagerClientResult<AccountsServerStatus> {
        //request status, await until receive, can has some common timeout
        let (tx, rx) = std::sync::mpsc::channel();
        // Send request to backend thread
        self.sender
            .send(BackendRequest::GetServerStatus { response: tx })
            .expect("Backend thread has shut down");

        // Wait for response (with timeout)
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(res) => res,
            Err(_) => Err(AccountsManagerClientError::Timeout), // Make sure you define this error variant
        }
    }


    pub fn request_register_new_account(&self, register_data: RegisterData) -> AccountsManagerClientResult<()> {
        //request status, await until receive, can has some common timeout
        let (tx, rx) = std::sync::mpsc::channel();
        // Send request to backend thread
        self.sender
            .send(BackendRequest::RegisterNewAccount { register_data, response: tx })
            .expect("Backend thread has shut down");

        // Wait for response (with timeout)
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(res) => res,
            Err(_) => Err(AccountsManagerClientError::Timeout), // Make sure you define this error variant
        }
    }
}
