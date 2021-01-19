use crate::bindable::Bindable;

struct BindGroup<'a> {
    resources: Vec<Box<dyn Bindable<'a>>>,
}
