use std::{
    collections::HashMap,
    mem::MaybeUninit,
    sync::{atomic::AtomicU8, Arc, RwLock},
};

use crossbeam_channel::{Receiver, Sender};
use simplemad::Decoder;

use self::read::RemoteStream;

mod read;

#[derive(Clone)]
pub struct Senders(pub Arc<RwLock<Vec<Sender<StreamPacket>>>>);

impl Senders {
    pub fn push(&self, sender: Sender<StreamPacket>) {
        self.0.write().expect("not poisoned").push(sender);
    }
}

pub struct Stream {
    pub count: Arc<AtomicU8>,
    pub senders: Senders,
}

impl Stream {
    pub fn start(&self, url: &str) {
        println!("Starting stream: {}", url);
        let count = self.count.clone();
        let url = url.to_string();
        let senders = self.senders.clone();
        std::thread::spawn(move || {
            let remote = RemoteStream::new(&url, senders.clone());
            let Ok(remote) = remote else {
                println!("Failed to start stream: {}", remote.err().expect("error expected"));
                return;
            };
            let Ok(decoder) = Decoder::decode(remote) else {
                println!("Failed to start stream: {}", url);
                for sender in senders.0.read().expect("not poisoned").iter() {
                    let _ = sender.send(StreamPacket::Close);
                }
                return;
            };
            for decoding_result in decoder {
                if count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
                    println!("no listeners, shutting down stream");
                    break;
                }
                match decoding_result {
                    Err(_) => {} // error!("Error: {:?}", e),
                    Ok(frame) => {
                        let mut samples: Vec<alto::Mono<f32>> = Vec::new();
                        for i in 0..frame.samples[0].len() {
                            samples.push(alto::Mono {
                                center: (frame.samples[0][i].to_f32()
                                    + frame.samples[1][i].to_f32())
                                    / 2.0_f32,
                            });
                        }
                        let mut delete = false;
                        for sender in senders.0.read().expect("not poisoned").iter() {
                            if let Err(e) = sender.send(StreamPacket::Data(
                                samples.clone(),
                                frame.sample_rate as i32,
                            )) {
                                println!("Failed to send data: {}", e);
                                delete = true;
                            }
                        }
                        if delete {
                            senders
                                .0
                                .write()
                                .expect("not poisoned")
                                .retain(|s| s.send(StreamPacket::Check).is_ok());
                        }
                    }
                }
            }
        });
    }
}

pub enum StreamPacket {
    Data(Vec<alto::Mono<f32>>, i32),
    Title(String),
    Close,
    Check,
}

pub struct StreamListener {
    pub receiver: Receiver<StreamPacket>,
    pub count: Arc<AtomicU8>,
}

impl Drop for StreamListener {
    fn drop(&mut self) {
        println!("stream listener dropped");
        self.count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        Streams::get()
            .write()
            .expect("not poisoned")
            .iter()
            .for_each(|(_, stream)| {
                stream
                    .senders
                    .0
                    .write()
                    .expect("not poisoned")
                    .retain(|s| s.send(StreamPacket::Check).is_ok());
            });
    }
}

pub struct Streams;

impl Streams {
    pub fn get() -> Arc<RwLock<HashMap<String, Stream>>> {
        static mut SINGLETON: MaybeUninit<Arc<RwLock<HashMap<String, Stream>>>> =
            MaybeUninit::uninit();
        static mut INIT: bool = false;

        unsafe {
            if !INIT {
                SINGLETON.write(Arc::new(RwLock::new(HashMap::new())));
                INIT = true;
            }
            SINGLETON.assume_init_ref().clone()
        }
    }

    pub fn listen(url: String) -> StreamListener {
        let (sender, receiver) = crossbeam_channel::unbounded();
        if let Some(stream) = Self::get().read().expect("not poisoned").get(&url) {
            println!("using existing stream");
            if stream
                .count
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
                == 0
            {
                stream.start(&url);
            }
            stream.senders.push(sender);
            return StreamListener {
                receiver,
                count: stream.count.clone(),
            };
        }
        println!("creating new stream");
        let stream = Stream {
            count: Arc::new(AtomicU8::new(1)),
            senders: Senders(Arc::new(RwLock::new(vec![sender]))),
        };
        stream.start(&url);
        let sl = StreamListener {
            receiver,
            count: stream.count.clone(),
        };
        Self::get()
            .write()
            .expect("not poisoned")
            .insert(url, stream);
        sl
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let receiver = super::Streams::listen(
            "http://pulseedm.cdnstream1.com:8124/1373_128".to_string(),
        );
        std::thread::sleep(std::time::Duration::from_secs(3));
        drop(receiver);
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
