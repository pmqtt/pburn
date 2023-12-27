use std::io::Bytes;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
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
    pub(crate) connection: Option<Connection>,
    pub(crate) timeout: Option<Duration>
}
impl Clone for MqttMessageFromBroker{
    fn clone(&self) -> Self {
        MqttMessageFromBroker{
            topic: self.topic.clone(),
            payload: self.payload.clone(),
            connection: None,
            timeout: self.timeout.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.payload = source.payload.clone();
        self.topic = source.topic.clone();
        self.connection = None;
        self.timeout = source.timeout.clone();
    }
}



pub struct MqttBroker{
    client: Client,
    connection: Option<Connection>,
    timeout: Option<Duration>,
    get_message_thread: Option<JoinHandle<MqttMessageFromBroker>>
}
impl MqttBroker{
    pub fn new(host: &String, port: u16, timeout: Option<Duration>) -> MqttBroker{
        let uuid = Uuid::new_v4();
        let mut mqttoptions = MqttOptions::new(uuid.to_string(), host, port );
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        let (mut client, mut connection) = Client::new(mqttoptions, 10);
        await_connection_to_broker(&mut connection);
        MqttBroker{
            client: client,
            connection: Some(connection),
            get_message_thread: None,
            timeout: timeout
        }
    }

    pub fn subscribe(&mut self, topic: &String){
        println!("Subscripe for topic:{}",topic);
        self.client.subscribe(topic,QoS::AtMostOnce);
        await_subscription(self.connection.as_mut().unwrap());
        let mut connection = self.connection.take().expect("Connection already in use");
        let mut timeout: Option<Duration> = self.timeout.take();
        let handle = thread::spawn(move ||{
            let mut time_spwan = Duration::from_secs(0);
            let mut start_time = std::time::Instant::now();
            loop {
                if let Ok(notification) = connection.recv() {
                    match notification {
                        Ok(v) => {
                            match v {
                                Event::Incoming(inc) => match inc {
                                    Incoming::Publish(msg) => {
                                        return MqttMessageFromBroker {
                                            topic: msg.topic.clone(),
                                            payload: String::from_utf8(msg.payload.to_vec()).unwrap(),
                                            connection: Some(connection),
                                            timeout: timeout
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
                let end_time = start_time.elapsed();
                time_spwan = time_spwan + end_time;
                match timeout {
                    None => {}
                    Some(out) => {
                        if time_spwan > out {
                            return MqttMessageFromBroker{
                                timeout: Some(out),
                                topic:"".to_string(),
                                payload:"".to_string(),
                                connection: Some(connection)
                            };
                        }
                    }
                }
                start_time = std::time::Instant::now();
            }
        });
        self.get_message_thread = Some(handle);
    }

    pub fn send(&mut self,topic: &String, payload: &String){
        println!("Send topic:{} with payload: {}" ,topic,payload);

        let res = self.client.publish(topic, QoS::AtMostOnce, false, payload.clone());
        match res{
            Ok(x) => {
                println!("OK");
            }
            Err(v) => {
                println!("{:?}",v);
            }
        }
        await_publish_is_send(self.connection.as_mut().unwrap());
    }

    pub fn get_message(&mut self) -> Result<MqttMessageFromBroker,String>{
        match self.get_message_thread.take(){
            None => {
                panic!("None");
            }
            Some(v) => {
                match v.join(){
                    Ok(mut x) => {
                        self.connection = Some(x.connection.take().expect("Not exist"));
                        self.timeout = x.timeout.take();
                        if x.topic == "".to_string(){
                            return Err("Timeout".to_string());
                        }
                        return Ok(x.clone());
                    }
                    Err(_) => {
                        panic!();
                    }
                }
            }
        }
    }
}