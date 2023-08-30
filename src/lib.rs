use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct OutOfBoundsError {
  size: usize,
  i: usize
}

pub struct Index<const N: usize> {
  pub i: usize
}

pub trait SmooFun<const N: usize> {
  fn apply(&self, args: [f64; N]) -> f64;
  fn derivative(&self, i: Index<N>) -> Box<dyn SmooFun<N>>;
}

impl<const N: usize> TryFrom<usize> for Index<N> {
  type Error = OutOfBoundsError;

  fn try_from(i: usize) -> Result<Self, Self::Error> {
    if i < N {
      Ok(Index { i })
    } else {
      Err(OutOfBoundsError::new(N, i))
    }
  }
}

impl OutOfBoundsError {
  pub fn new(size: usize, i: usize) -> OutOfBoundsError {
    OutOfBoundsError { size, i }
  }
  pub fn message(&self) -> String {
    format!("Index {} must be less than size {}", self.i, self.size)
  }
}

impl Debug for OutOfBoundsError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{}", self.message())
  }
}

impl Display for OutOfBoundsError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{}", self.message())
  }
}

impl Error for OutOfBoundsError {

}

