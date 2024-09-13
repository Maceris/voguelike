
pub trait Action {
    fn precondition() -> bool;
    fn perform();
}
