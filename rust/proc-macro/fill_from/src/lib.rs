pub trait Fill<T> {
    fn generate<T>(input: T) -> Self;
}
