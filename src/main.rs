// src/main.rs

// Copyright 2024 Kriyaetive Verse Private Limited
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Author: Sridhar Ananthakrishnan <itsmycodehub@gmail.com>

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use tokio::time::{self, Duration};
use futures::Stream;
use std::pin::Pin;

type BoxedStream = Pin<Box<dyn Stream<Item = Result<Body, hyper::Error>> + Send>>;

async fn handle_request(_req: Request<Body>) -> Result<Response<BoxedStream>, Infallible> {
    // Simulating a video/audio stream by sending chunks of data over time.
    let stream = async_stream::try_stream! {
        for i in 0..10 {
            yield Body::from(format!("Chunk #{}\n", i));
            time::sleep(Duration::from_secs(1)).await;
        }
    };

    Ok(Response::new(Box::pin(stream) as BoxedStream))
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("FluxStream server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

