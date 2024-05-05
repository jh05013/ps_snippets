pub mod queue_min_mod {
	use std::collections::VecDeque;

	/// A queue supporting minimum-in-queue.
	#[derive(Clone, Debug, Default)]
	pub struct QueueMin<T: PartialOrd> {
		q: VecDeque<(T, usize)>,
		i: usize, j: usize,
	}

	impl<T: PartialOrd> QueueMin<T> {
		/// Initializes an empty queue.
		pub fn new() -> Self { Self { q: VecDeque::new(), i: 0, j: 0 } }

		/// Returns the number of elements.
		pub fn len(&self) -> usize { self.j - self.i }
		/// Returns `true` iff empty.
		pub fn is_empty(&self) -> bool { self.i == self.j }

		/// Pushes `v` into the queue.
		pub fn push(&mut self, v: T) {
			while let Some(y) = self.q.back() {
				if y.0 >= v { self.q.pop_back(); }
				else { break; }
			}
			self.q.push_back((v, self.j));
			self.j += 1;
		}

		/// Pops from the queue and returns `true` if the queue is not empty;
		/// otherwise returns `false`.
		pub fn pop(&mut self) -> bool {
			let y = match self.q.front() {
				Some(y) => y, None => return false
			};
			if y.1 == self.i { self.q.pop_front(); }
			self.i += 1;
			true
		}

		/// Returns the minimum value in the queue, if any; otherwise returns `None`.
		pub fn get(&self) -> Option<&T> { self.q.front().map(|(x, _)| x) }

		/// Returns the most recent value inserted into the queue.
		pub fn most_recent(&self) -> Option<&T> { self.q.back().map(|(x, _)| x) }
	}
}
pub use queue_min_mod::QueueMin;
