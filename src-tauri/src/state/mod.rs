use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};
use tauri::{AppHandle, Manager};

mod thread_manager;
use thread_manager::thread_object::{Message, TreadObject};

#[derive(Debug, Default, Serialize, Clone)]
struct MyHashMap {
    hashmap: Arc<Mutex<HashMap<Message, u32>>>,
}

pub struct ThreadManager {
    threads: Arc<Mutex<HashMap<u32, TreadObject>>>,
    next_id: Arc<Mutex<u32>>,
    sender: Sender<Message>,
}

impl ThreadManager {
    pub fn new(app: AppHandle) -> Self {
        let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let (tx_fifo, rx_fifo): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let app_clone = app.clone();
        

        let fifo_collection: Arc<Mutex<HashMap<u32, u32>>> = Arc::new(Mutex::new(HashMap::new()));
        let fifo_collection_clone = fifo_collection.clone();
        let _fifo_handle = thread::spawn(move || {
            for received in rx_fifo {
                let fifo = "fifo";
                {
                    let mut fifo_collection_locked = fifo_collection_clone.lock().unwrap();
                    fifo_collection_locked.insert(received.id.clone(), received.count.clone());
                }
                let fifo_clone = fifo_collection_clone.clone();
                send_serialised_mutex(fifo_clone, app.clone(), fifo).unwrap();
                let _ = tx.send(received);
            }
        });
        
        // No need to clone fifo_collection here again
        let fifo_collection_clone = fifo_collection.clone();
        let _receiver_handle = thread::spawn(move || {
            let collection: HashMap<u32, u32> = HashMap::new();
            
            let mut collection_clone = collection;
            for received in rx {
                let hashmap = "hashmap";
                collection_clone.insert(received.id.clone(), received.count.clone());
                {
                    let mut fifo_collection_locked = fifo_collection_clone.lock().unwrap();
                    fifo_collection_locked.remove(&received.id.clone());
                    println!("fifo after rm: {:?}", fifo_collection_locked)
                }
                send_serialised(collection_clone.clone(), app_clone.clone(), hashmap).unwrap();
                let fifo_clone_for_send = fifo_collection.clone();
                send_serialised_mutex(fifo_clone_for_send, app_clone.clone(), "fifo").unwrap();
                thread::sleep(Duration::from_millis(400));
            }
        });

        Self {
            threads: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
            sender: tx_fifo,
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

fn send_serialised_mutex(hashmap_mutex: Arc<std::sync::Mutex<HashMap<u32, u32>>>, app: AppHandle, event: &str) -> Result<(), String> {
    let hashmap_locked = hashmap_mutex.lock().unwrap();
    let hashmap_data: Vec<_> = hashmap_locked
        .iter()
        .map(|(k, &v)| (k.clone(), v))
        .collect();

    println!("fifo {:?}", hashmap_data);
    match app.emit(event, &hashmap_data) {
        Ok(_) => {
            println!("fifo hashmap emitted successfully");
            Ok(())
        }
        Err(e) => {
            println!("Failed to emit event for hashmap: {:?}", e);
            Err(format!("Failed to emit event: {:?}", e))
        }
    }
}

fn send_serialised(hashmap: HashMap<u32, u32>, app: AppHandle, event: &str) -> Result<(), String> {
    let hashmap_data: Vec<_> = hashmap
        .iter()
        .map(|(k, &v)| (k.clone(), v))
        .collect();

    println!("hashmap {:?}", hashmap_data);
    match app.emit(event, &hashmap_data) {
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