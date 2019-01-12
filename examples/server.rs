extern crate actix_telegram;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate log;
extern crate serde_json;

use actix_telegram::actors::{telegram_server::*, TelegramApi};
use actix_telegram::application::UpdateHandler;
use actix_telegram::types::update::{Update, UpdateKind};
use actix_web::actix::{Actor, Addr, System};
use std::env;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

fn main() {
    env_logger::init();
    let sys = System::new("example");
    let token = env::var("TELEGRAM_TOKEN").unwrap();
    //    let print = App::new(print_update);
    let key = Key::new(env::var("KEY").unwrap(), KeyKind::PKCS8);
    let cert = Cert::new(env::var("CERTIFICATE_PEM").unwrap());
    let host = env::var("WEBHOOK_HOST").unwrap();
    let cert_and_key = CertAndKey::new(cert, key);
    let arc = Arc::new(Mutex::new(HashMap::new()));
    let _server = TelegramServer::new("127.0.0.1:59080".to_owned(), token, host, move || {
        let clone = arc.clone();
        Test { state: clone }
    })
    .set_workers(4)
    .set_certificate_and_key(cert_and_key, true)
    .start();
    sys.run();
}

struct Test {
    state: Arc<Mutex<HashMap<i64, ()>>>,
}

impl UpdateHandler for Test {
    fn handle(&self, update: Update, _: &Addr<TelegramApi>) -> Result<(), Update> {
        if let UpdateKind::Message(message) = update.kind() {
            if let Some(user) = message.from() {
                let mut hash_map = self.state.lock().unwrap();
                hash_map.insert(*user.id(), ());
            }
        }
        println!("{:?}", self.state);
        Ok(())
    }
}
