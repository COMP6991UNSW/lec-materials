use exam_q6_lib::channel;

fn main() {
	std::thread::scope(|scope| {
		let (mut send, mut recv) = channel();

		scope.spawn(move || {
			while let Some(num) = recv.recv() {
				println!("Thread got {num}!");
			}

			println!("Thread finished!");
		});

		for i in 1..=5 {
			println!("Sending {i}...");
			send.send(i);
		}

		println!("Sending finished!");
		drop(send);
	});
}
