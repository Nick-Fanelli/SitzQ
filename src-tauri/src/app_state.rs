use crate::connections::NetworkRemote;

use std::sync::Mutex;
use std::sync::Arc;

use tauri::State;
use uuid::Uuid;

pub type MultithreadedAppState = Arc<Mutex<AppState>>;

pub struct AppState {

    network_remotes: Mutex<Vec<NetworkRemote>>

}

impl AppState {

    pub fn new() -> Self {

        AppState {
            network_remotes: Default::default()
        }

    }

    fn generate_network_remote_uuid(&self, network_remotes: &Vec<NetworkRemote>) -> String {

        loop {
            let full_uuid = Uuid::new_v4();
            let short_uuid = full_uuid.simple().to_string();
            let uuid = short_uuid.chars().take(8).collect::<String>();

            let is_collision = network_remotes.iter().any(|remote| remote.uuid == uuid);

            if !is_collision {
                return uuid;
            }
        }

    }

    // fn get_network_remote(&self, uuid: String) -> Option<&NetworkRemote> {

    //     // let network_remotes = self.network_remotes.lock().unwrap();

    //     // let result = network_remotes.iter().find(|remote| remote.uuid == uuid);

    //     // return result;
    // }

    fn generate_network_remote(&mut self, address: String) -> String {

        let mut network_remotes = self.network_remotes.lock().unwrap();

        let uuid = self.generate_network_remote_uuid(&network_remotes);
        let remote = NetworkRemote::new(uuid.clone(), address);

        network_remotes.push(remote);

        return uuid;

    }

}

#[tauri::command]
pub fn generate_network_remote(address: String, state: State<MultithreadedAppState>) -> String { // Returns UUID to the remote
    
    let mut state = state.lock().unwrap();
    return state.generate_network_remote(address);

}

#[tauri::command]
pub fn connect_network_remote(state: State<MultithreadedAppState>) {

    let state = state.lock().unwrap();



}