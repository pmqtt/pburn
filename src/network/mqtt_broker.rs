use std::io::Bytes;
use std::thread;
use std::time::Duration;
use rumqttc::{Client, Connection, ConnectionError, Event, Incoming, MqttOptions, Outgoing, QoS};
use uuid::{Uuid, uuid};

fn await_connection_to_broker(connection: &mut Connection) {
    for (i, notification) in connection.iter().enumerate() {
        match notification {
            Ok(evt) => {
                match evt {
                    Event::Incoming(v) => {
                        match v {
                            Incoming::ConnAck(_) => { break }
                            _ => {}
                        }
                    }
                    Event::Outgoing(_) => {}
                }
            }
            Err(_) => {}
        }
    }
}

fn await_publish_is_send(connection: & mut Connection){
    for (i, notification) in connection.iter().enumerate() {
        match notification{
            Ok(evt) => {
                match evt {
                    Event::Incoming(v) => {}
                    Event::Outgoing(v) => {
                        match v{
                            Outgoing::Publish(v) => { break;}
                            _ => {}
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }
}

fn await_subscription(connection: & mut Connection){
    for (i, notification) in connection.iter().enumerate() {
        match notification{
            Ok(evt) => {
                match evt {
                    Event::Incoming(v) => {}
                    Event::Outgoing(v) => {
                        match v{
                            Outgoing::Subscribe(v) => { break;}
                            _ => {}
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }
}

pub struct MqttMessageFromBroker{
    pub(crate) topic: String,
    pub(crate) payload: String,
}
pub struct MqttBroker{
    client: Client,
    connection: Connection
}
impl MqttBroker{
    pub fn new(host: &String, port: u16) -> MqttBroker{
        let uuid = Uuid::new_v4();
        let mut mqttoptions = MqttOptions::new(uuid.to_string(), host, port );
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        let (mut client, mut connection) = Client::new(mqttoptions, 10);
        await_connection_to_broker(&mut connection);
        MqttBroker{
            client: client,
            connection: connection,
        }
    }

    pub fn subscribe(&mut self, topic: &String){
        self.client.subscribe(topic,QoS::AtMostOnce);
        await_subscription(&mut self.connection);
    }

    pub fn send(&mut self,topic: &String, payload: &String){
        let res = self.client.publish(topic, QoS::AtMostOnce, false, payload.clone());
        match res{
            Ok(x) => {
                println!("OK");
            }
            Err(v) => {
                println!("{:?}",v);
            }
        }
        await_publish_is_send(&mut self.connection);
    }

    pub fn get_message(&mut self) -> MqttMessageFromBroker{

        loop {
            if let Ok(notification) = self.connection.recv() {
                match notification {
                    Ok(v) => {
                        match v {
                            Event::Incoming(inc) => match inc {
                                Incoming::Publish(msg) => {
                                    return MqttMessageFromBroker {
                                        topic: msg.topic.clone(),
                                        payload: String::from_utf8(msg.payload.to_vec()).unwrap()
                                    };
                                }

                                _ => {}
                            },
                            Event::Outgoing(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }
}