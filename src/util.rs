// Copyright (C) 2014 The 6502-rs Developers
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. Neither the names of the copyright holders nor the names of any
//    contributors may be used to endorse or promote products derived from this
//    software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

// Wraps std::io::timer::Timer with functions that take a function that will be
// called once the timer has fired.
use std::comm::TryRecvError;
use std::io::timer::Timer;
use std::io::IoResult;
use std::time::duration::Duration;

pub struct FnTimer {
	period_done_sender:   Option<Sender<()>>
}

impl FnTimer{
	pub fn new() -> FnTimer {
		FnTimer {
			period_done_sender: None
		}
	}

	pub fn sleep(duration: Duration, fun: ||) {
		let mut timer = Timer::new().unwrap();

		timer.sleep(duration);
		fun();
	}

	pub fn oneshot<F: FnOnce() + Send>(duration: Duration, fun: F) {
		let mut timer = Timer::new().unwrap();

		spawn(move |:| {
			let receiver = timer.oneshot(duration);
			receiver.recv();
			fun();
		});
	}

	pub fn periodic<F: FnOnce() + Send>(&mut self, duration: Duration, fun: F) {
		let (period_done_sender, period_done_receiver) = channel();
		self.period_done_sender = Some(period_done_sender);

		let mut timer = Timer::new().unwrap();

		spawn(move |:| {
			let receiver = timer.periodic(duration);

			loop {
				let period_done_result = period_done_receiver.try_recv();

				match period_done_result {
					Ok(())                   => return,
					Err(TryRecvError::Empty) => { },
					Err(error)               => panic!(error)
				}

				receiver.recv();
				fun(); // ERROR: fun has trait FnOnce but is called multiple times.
			}
		});
	}

	// TODO akeeton: destructor calls period_done_sender.send().
}
