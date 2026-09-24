use libs_modeling::identifier::Identifier;
use libs_modeling::id_provider::IdProvider;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventId(Uuid);

impl Identifier<Uuid> for EventId {
    fn generate(id_provider: &dyn IdProvider<Uuid>) -> EventId {
        EventId(id_provider.generate())
    }

    fn new(id: Uuid) -> Self {
        Self(id)
    }
}
