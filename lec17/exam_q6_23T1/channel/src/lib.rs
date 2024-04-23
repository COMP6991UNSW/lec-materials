pub fn channel() -> (Sender<i32>, Receiver<i32>) {
	let buffer  = Box::into_raw(Box::new(None::<i32>));
	let hung_up = Box::into_raw(Box::new(false));

	let sender   = Sender   { buffer, hung_up };
	let receiver = Receiver { buffer, hung_up };

	(sender, receiver)
}

pub struct Sender<T> {
	buffer:  *mut Option<T>,
	hung_up: *mut bool,
}

pub struct Receiver<T> {
	buffer:  *mut Option<T>,
	hung_up: *mut bool,
}

impl<T> Sender<T> {
	pub fn send(&mut self, value: T) -> Option<()> {
		if unsafe { *self.hung_up } {
			return None;
		}

		// wait until the channel is empty...
		loop {
			let value = unsafe { std::ptr::read(self.buffer) };
			if value.is_none() { break; }
			std::mem::forget(value);
		}

		// send the value into the shared buffer
		unsafe { std::ptr::write(self.buffer, Some(value)); }

		Some(())
	}
}

impl<T> Receiver<T> {
	pub fn recv(&mut self) -> Option<T> {
		loop {
			if unsafe { *self.hung_up } {
				return None;
			}

			// wait until the value exists...
			if let Some(value) = unsafe { std::ptr::read(self.buffer) } {
				// clear the channel for the next message
				unsafe { std::ptr::write(self.buffer, None); }

				return Some(value);
			}
		}
	}
}

unsafe impl<T: Send> Send for Sender<T> {}
unsafe impl<T> Send for Receiver<T> {}

impl<T> Drop for Sender<T> {
	fn drop(&mut self) {
		unsafe { *self.hung_up = true; }
	}
}

impl<T> Drop for Receiver<T> {
	fn drop(&mut self) {
		unsafe { *self.hung_up = true; }
	}
}

