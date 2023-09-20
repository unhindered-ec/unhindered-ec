pub trait TypeEq {
    type This: ?Sized;
}

impl<T: ?Sized> TypeEq for T {
    type This = Self;
}
