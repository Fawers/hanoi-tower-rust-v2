pub trait TakeInput {
    type Output;
    fn take_input(&self) -> Self::Output;
}

