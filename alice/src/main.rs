// See https://blockchain.info/api/exchange_rates_api.

extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::error::Error;
use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use hyper::Chunk;
use tokio_core::reactor::Core;


fn main() {
	println!("Started");


	let result = do_main();

	println!("Result = {:?}", result);




    println!("Finished");
}


fn do_main() -> Result<(), Box<Error>> {




	let url = "http://blockchain.info/ticker";

	println!("url = {}", url);

	let mut core = Core::new()?;
	let client = Client::new(&core.handle());
	let uri = url.parse()?;

	let work = client.get(uri).and_then(|res| {
	    println!("Response: {}", res.status());
	    println!("Version: {}", res.version());

		res.body().concat2().and_then(move |body: Chunk| {
			println!("{:?}", body);	
		    Ok(())
		})
	});

	core.run(work)?;


	Ok(())
}
