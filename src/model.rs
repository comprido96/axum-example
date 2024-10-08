//! Simplistic Model layer
//! (with mock-store layer in-memory)

use crate::{ctx::Ctx, Error, Result};
 use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    id: u64,
    pub cid: u64,
    title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    title: String,
}

// only for quick local prototype,
// bc it will grow infinitely in-memory
// implements Clone bc mc will be an application state
// however, it does not clone the vector, just the arc ref
#[derive(Clone)]
pub struct  ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create(
        &self,
        ctx: Ctx,
        ticket_fc: TicketForCreate
    ) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(
        &self,
        _ctx: Ctx,
    ) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        // this is gonna clone Option and its content, so if Option is none it will
        // be filtered out
        let list_tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(list_tickets)
    }

    pub async fn delete(
        &self,
        _ctx: Ctx,
        id: u64
    ) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}




























