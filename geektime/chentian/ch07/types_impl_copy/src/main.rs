fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
}

fn types_not_impl_copy_trait() {
    is_copy::<str>();
}

fn main() {
    types_impl_copy_trait();
}
