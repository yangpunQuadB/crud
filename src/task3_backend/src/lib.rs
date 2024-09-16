use ic_cdk_macros::{update, query};
use std::collections::HashMap;
use std::sync::Mutex;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct Message {
    id: u64,
    text: String,
}


thread_local! {
    static STATE: Mutex<Crud> = Mutex::new(Crud::default());
}


#[derive(Default)]
pub struct Crud {
    messages: HashMap<u64, Message>,
    next_id: u64,
}

#[update]
pub fn create(text: String) {
    STATE.with(|state| {
        let mut state = state.lock().unwrap();     
        let id = state.next_id;
        state.next_id += 1;
        state.messages.insert(id, Message { id, text });
    })
}
// Get all mssg
#[query]
pub fn read(id: u64) -> Message {
    STATE.with(|state| {
        let state = state.lock().unwrap();
        state.messages.get(&id).cloned().unwrap_or(Message {
            id: 0, 
            text: "Message not found.".to_string(),
        })
    })
}

// Update mssg
#[update]
pub fn update(id: u64,text:String) {
    STATE.with(|state| {
        let mut state = state.lock().unwrap();
        if let Some(task) = state.messages.get_mut(&id) {
            task.text = text;
        }
    });
}

// Remove a message
#[update]
pub fn delete(id: u64) {
    STATE.with(|state| {
        let mut state = state.lock().unwrap();
        state.messages.remove(&id);
    });
}
#[query]
pub fn get_posts(page_number: u64, page_size: u64) -> Vec<Message> {
    STATE.with(|state| {
        let state = state.lock().unwrap();
        let mut messages: Vec<_> = state.messages.values().cloned().collect();
        messages.sort_by(|a, b| b.id.cmp(&a.id));
        let start = page_number * page_size;
        messages.into_iter().skip(start as usize).take(page_size as usize).collect()
    })
}

#[query]
pub fn latest_post() -> Vec<Message> {
    STATE.with(|state| {
        let state = state.lock().unwrap();
        let mut messages: Vec<_> = state.messages.values().cloned().collect();
        messages.sort_by(|a, b| b.id.cmp(&a.id));
        messages.into_iter().take(10).collect()
    })
}