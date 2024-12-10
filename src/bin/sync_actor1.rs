use actix::prelude::*;
use tokio;

struct MySyncActor {
    count: usize,
}

impl Actor for MySyncActor {
    type Context = SyncContext<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);

impl Handler<Ping> for MySyncActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut SyncContext<Self>) -> Self::Result {
        println!("Thread {:?} received Ping {}", std::thread::current().id(), msg.0);
        self.count += msg.0;
        self.count
    }
}

#[actix::main]
async fn main() {
    // start new actor
    let addr = SyncArbiter::start(8, || MySyncActor { count: 0 });

    // Send message and collect all futures
    let mut futures = vec![];
    for i in 0..10 {
        let res = addr.send(Ping(i));
        futures.push(res);
    }

    // Wait for all futures to complete
    for future in futures {
        match future.await {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Error: {}", err),
        }
    }

    // stop system and exit
    println!("All messages processed. Stopping system...");
    System::current().stop();
}