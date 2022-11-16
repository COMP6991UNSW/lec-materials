use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::{mpsc, Arc};
use std::task::{Poll, Context, Waker, Wake};

// async, await
// epoll

async fn website_request(url: &'static str) -> String {
    reqwest::get(url).await
        .unwrap()
        .text()
        .await
        .unwrap()
}

enum WebsiteRequest {
    Init(&'static str),
    SentRequest(mpsc::Receiver<String>),
    RequestDone,
}

struct OurWaker;

impl Wake for OurWaker {
    fn wake(self: Arc<Self>) {}
}

impl Future for WebsiteRequest {
    type Output = String;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match self.deref() {
            &WebsiteRequest::Init(url) => {
                let (sender, receiver) = mpsc::channel();
                {
                    let waker = cx.waker().clone();
                    std::thread::spawn(move || {
                        let response = reqwest::blocking::get(url) // PART 2
                            .unwrap() // PART 3 (END)
                            .text()
                            .unwrap();

                        sender.send(response).unwrap();
                        waker.wake();
                    });
                }

                *self = Self::SentRequest(receiver);
                
                Poll::Pending
            }
            WebsiteRequest::SentRequest(receiver) => {
                match receiver.try_recv() {
                    Ok(response) => {
                        *self = Self::RequestDone;
                        Poll::Ready(response)
                    }
                    Err(_) => {
                        Poll::Pending
                    }
                }
            }
            WebsiteRequest::RequestDone => {
                // panic!("The request was already completed!")
                Poll::Pending
            }
        }
    }
}

fn get_website(url: &str, name: &str) -> String {
    // PART 1
    println!("Sending request to {name}...");
    let response = reqwest::blocking::get(url) // PART 2
        .unwrap() // PART 3 (END)
        .text()
        .unwrap();

    println!("Received response from {name}");
    response 
}

fn get_website_future<'name>(websites: &[(&'static str, &'name str)]) -> Vec<(&'name str, String)> {
    let mut tasks = websites.into_iter()
        .map(|(url, name)| {
            (WebsiteRequest::Init(url), name)
        })
        .collect::<Vec<_>>();

    let waker = Arc::new(OurWaker).into();
    let mut context = Context::from_waker(&waker);

    let mut responses = vec![];

    loop {
        if responses.len() == tasks.len() { break }

        for (task, name) in tasks.iter_mut() {
            let poll = Pin::new(task).poll(&mut context);
        
            match poll {
                Poll::Ready(response) => {
                    responses.push((**name, response));
                }
                Poll::Pending => {}
            }
        }
    }

    responses
}

fn main() {
    let responses = get_website_future(&vec![
        ("https://rust-lang.org/", "rust-lang"),
        ("https://en.cppreference.com/w/", "cppreference"),
        ("https://www.python.org/", "python.org"),
    ]);

    println!("{responses:?}");
}
