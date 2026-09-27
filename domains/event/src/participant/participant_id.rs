use libs_modeling::id_provider::IdProvider;
use libs_modeling::identifier::Identifier;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParticipantId(Uuid);

impl Identifier<Uuid> for ParticipantId {
    fn generate(id_provider: &dyn IdProvider<Uuid>) -> Self {
        Self(id_provider.generate())
    }

    fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl ParticipantId {
    pub fn value(&self) -> Uuid {
        self.0
    }
}
