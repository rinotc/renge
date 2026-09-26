use super::shared::EventView;
use askama::Template;
use domains_event::event::Event as DomainEvent;

#[derive(Template)]
#[template(path = "event_index.askama.html")]
pub(crate) struct EventIndexTemplate {
    page_title: &'static str,
    events: Vec<EventView>,
    current_page: u32,
    has_previous: bool,
    previous_page: u32,
    has_next: bool,
    next_page: u32,
}

impl EventIndexTemplate {
    pub(crate) fn new(
        events: Vec<DomainEvent>,
        current_page: u32,
        has_previous: bool,
        has_next: bool,
    ) -> Self {
        Self {
            page_title: "イベント一覧",
            events: events.iter().map(EventView::from).collect(),
            current_page,
            has_previous,
            previous_page: current_page.saturating_sub(1),
            has_next,
            next_page: current_page.saturating_add(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use askama::Template;

    #[test]
    fn event_index_renders_empty_state() {
        let html = EventIndexTemplate::new(Vec::new(), 1, false, false)
            .render()
            .unwrap();

        assert!(html.contains("まだイベントがありません。右のフォームから作成できます。"));
    }
}
