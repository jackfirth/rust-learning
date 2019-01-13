#![feature(associated_type_defaults)]

pub enum TransducerState<C, E, H> {
  Consume(C),
  Emit(E),
  HalfClosed(H),
  Done,
}

pub enum HalfClosedTransducerState<H> {
  HalfClosed(H),
  Done,
}

pub trait Transducer<T,U> {
  type ConsumeState;
  type EmitState;
  type HalfClosedState;

  type Output = TransducerState<Self::ConsumeState, Self::EmitState, Self::HalfClosedState>;
  type HalfClosedOutput = HalfClosedTransducerState<Self::HalfClosedState>;

  fn start(&self) -> Self::Output;
  fn consume (&self, state: Self::ConsumeState, input: T) -> Self::Output;
  fn emit (&self, state: Self::EmitState) -> (Self::Output, U);
  fn emit_while_half_closed (&self, state: Self::HalfClosedState) -> (Self::HalfClosedOutput, U);
  fn close (&self, state: Self::Output);
  fn half_close (&self, state: Self::ConsumeState) -> Self::HalfClosedOutput;
}

fn main() {
    println!("Hello, world!");
}
