use std::{collections::HashMap, sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread, time::Duration};
use serde::Serialize;
use tauri::{AppHandle, Manager};

mod thread_manager;
use thread_manager::thread_object::{TreadObject, Message};

#[derive(Debug, Default, Serialize, Clone)]
struct MyHashMap {
    hashmap: Arc<Mutex<HashMap<Message, u32>>>,
}

impl MyHashMap {
    fn new() -> Self {
        Self { hashmap: Arc::new(Mutex::new(HashMap::new())) }
    }

    fn send_serialised(&self, app: AppHandle) -> Result<(), String> {
        let serialized_hashmap = {
            let hashmap_locked = self.hashmap.lock().unwrap();
            hashmap_locked.clone() // Clone the underlying data, not the guard
        };

        match app.emit("hashmap", serialized_hashmap) {
            Ok(_) => {
                println!("hashmap emitted successfully");
                Ok(())
            }
            Err(e) => {
                println!("Failed to emit event for hashmap: {:?}", e);
                Err(format!("Failed to emit event: {:?}", e))
            }
        }
    }
}

pub struct ThreadManager {
    threads: Arc<Mutex<HashMap<u32, TreadObject>>>,
    next_id: Arc<Mutex<u32>>,
    sender: Sender<Message>,
}

impl ThreadManager {
    pub fn new(app: &tauri::App) -> Self {
        let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

        let app = Arc::new(app);
        // Spawn the default receiver thread
        let receiver_handle = thread::spawn(move || {
            let collection = MyHashMap::new();
            for received in rx {
                let mut hashmap_locked = collection.hashmap.lock().unwrap();
                hashmap_locked.entry(received).and_modify(|e| *e += 1).or_insert(1);

                let serialized_hashmap: HashMap<Message, u32> = hashmap_locked.clone();
                match app.emit("hashmap", serialized_hashmap) {
                    Ok(_) => println!("hashmap {:?}", hashmap_locked),
                    Err(e) => println!("Failed to emit event for hashmap"),
                }
                thread::sleep(Duration::from_secs(1));  // rate-limit simulation
            }
        });

        Self {
            threads: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
            sender: tx,
        }
    }

    pub fn add_thread(&self, rate: u32, app: AppHandle) -> u32 {
        let id = {
            let mut next_id = self.next_id.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let sender_clone = self.sender.clone();
        let thread_object = TreadObject::new(id, rate, app, sender_clone);
        self.threads.lock().unwrap().insert(id, thread_object);
        id
    }

    pub fn stop_thread(&self, id: u32) -> bool {
        let mut threads = self.threads.lock().unwrap();
        if let Some(mut thread_object) = threads.remove(&id) {
            thread_object.stop();
            true
        } else {
            false
        }
    }

    pub fn get_thread_ids(&self) -> Vec<u32> {
        self.threads.lock().unwrap().keys().cloned().collect()
    }
}
