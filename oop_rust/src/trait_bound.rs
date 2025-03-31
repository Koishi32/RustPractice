use crate::trait_object::Draw;
pub struct Screen_trait_bound<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen_trait_bound<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}