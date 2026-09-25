use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Participant {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub attendance: String,
}

impl Participant {
    pub fn new(id: Uuid, name: String, email: String, attendance: String) -> Self {
        Self {
            id,
            name,
            email,
            attendance,
        }
    }
}
