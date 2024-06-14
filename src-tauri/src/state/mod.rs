use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex, MutexGuard,
    },
    thread, time::Duration,
};
use tauri::{AppHandle, Manager};

mod thread_manager;
use thread_manager::thread_object::{Message, TreadObject};

#[derive(Debug, Default, Serialize, Clone)]
struct MyHashMap {
    hashmap: Arc<Mutex<HashMap<Message, u32>>>,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            hashmap: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn add(&self, massage: Message)  {
        let mut hashmap_locked = self.hashmap.lock().unwrap();
                hashmap_locked
                    .entry(massage)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);

    }
    fn remove(&self, message: Message) {
        let mut hashmap_locked = self.hashmap.lock().unwrap();
        hashmap_locked.remove(&message);
    }
    
    fn lock(&self) -> MutexGuard<HashMap<Message, i32>> {
        self.hashmap.lock().unwrap()
    }

    fn send_serialised(&self, app: AppHandle, event: &str) -> Result<(), String> {
        let hashmap_locked = self.hashmap.lock().unwrap();
        let hashmap_data: Vec<_> = hashmap_locked.iter().map(|(k, &v)| (k.clone(), v)).collect();

        println!("{:?}",hashmap_data);
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

        // Spawn the default receiver thread
        let fifo_collection = MyHashMap::new();

        let _fifo_handle = thread::spawn(move || {
            for received in rx_fifo {
                let fifo = "fifo";
                {
                    let mut fifo_collection_locked = fifo_collection.lock().unwrap();
                    fifo_collection_locked.add(received.clone());
                    let _ = fifo_collection_locked.send_serialised(app.clone(), fifo);
                }
                let _ = tx.send(received);
            }
        });

        // Spawn the default receiver thread
        let _receiver_handle = thread::spawn(move || {
            let collection = MyHashMap::new();
            for received in rx {
                let hashmap = "hashmap";
                collection.add(received);
                {
                    let mut fifo_collection_locked = fifo_collection.lock().unwrap();
                    fifo_collection_locked.remove(received.clone());
                }
                let _ = collection.send_serialised(app_clone.clone(), hashmap);
                thread::sleep(Duration::from_secs(2));
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
