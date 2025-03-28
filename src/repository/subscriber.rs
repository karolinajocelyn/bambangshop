use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

lazy_static! {
    pub static ref SUBSCRIBERS: DashMap<String, Subscriber> = DashMap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
    pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
        let subscriber_value = subscriber.clone();
        if SUBSCRIBERS.get(product_type).is_none() {
            SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
        }

        SUBSCRIBERS.get(product_type).unwrap().insert(subscriber.url.clone(), subscriber_value);
        return subscriber;
    }
}