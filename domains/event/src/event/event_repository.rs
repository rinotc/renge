use libs_paging::paging::offset_paging::OffsetPaging;
use crate::event::Event;
use crate::event::event_id::EventId;

pub trait EventRepository {

    fn find_by_id(&self, id: &EventId) -> Option<Event>;

    fn list(&self, paging: OffsetPaging);

    fn insert(&self, event: &Event);

    fn update(&self, event: &Event);

    fn delete(&self, id: &EventId);
}