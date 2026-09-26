pub(crate) use super::shared::Attendance;
use super::shared::{AttendanceOption, ParticipantView, attendance_options};
use askama::Template;
use infra_postgres_renge_orm::orm::participants as participant;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "participant_row.askama.html")]
pub(crate) struct ParticipantRowTemplate {
    event_id: Uuid,
    person: ParticipantView,
    attendance_options: [AttendanceOption; 3],
}

impl ParticipantRowTemplate {
    pub(crate) fn new(event_id: Uuid, person: &participant::Model) -> Self {
        Self {
            event_id,
            person: ParticipantView::from(person),
            attendance_options: attendance_options(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::shared::AttendanceView;
    use super::*;
    use askama::Template;

    #[test]
    fn participant_row_selects_current_attendance() {
        let html = ParticipantRowTemplate {
            event_id: Uuid::nil(),
            person: ParticipantView {
                id: Uuid::nil(),
                name: "参加者".to_owned(),
                email: "person@example.com".to_owned(),
                status: AttendanceView::from(Attendance::Attending),
            },
            attendance_options: attendance_options(),
        }
        .render()
        .unwrap();

        assert!(html.contains("value=\"attending\" selected"));
    }
}
