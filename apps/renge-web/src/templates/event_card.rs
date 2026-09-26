use super::shared::EventView;
use askama::Template;
use domains_event::event::Event as DomainEvent;

#[derive(Template)]
#[template(path = "event_card.askama.html")]
pub(crate) struct EventCardTemplate {
    event: EventView,
}

impl EventCardTemplate {
    pub(crate) fn from_domain(event: &DomainEvent) -> Self {
        Self {
            event: EventView::from(event),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use askama::Template;
    use uuid::Uuid;

    #[test]
    fn event_card_escapes_user_input() {
        let html = EventCardTemplate {
            event: EventView {
                id: Uuid::nil(),
                title: "<script>alert(1)</script>".to_owned(),
                description: "説明".to_owned(),
                has_description: true,
                starts_at: "2026年9月7日 12:00".to_owned(),
                location: "会場".to_owned(),
            },
        }
        .render()
        .unwrap();

        assert!(html.contains("&#60;script&#62;alert(1)&#60;/script&#62;"));
        assert!(!html.contains("<script>alert(1)</script>"));
    }
}
