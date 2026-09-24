use crate::id_provider::IdProvider;

pub trait Identifier<ID> {
    fn generate(id_provider: &dyn IdProvider<ID>) -> Self;
    
    fn new(id: ID) -> Self;
}


