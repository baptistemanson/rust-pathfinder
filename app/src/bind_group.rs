use crate::bindable::Bindable;

#[allow(dead_code)]
struct BindGroup<'a> {
    resources: Vec<Box<dyn Bindable<'a>>>,
}
