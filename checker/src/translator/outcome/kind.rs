use tracing::info;

#[derive(Debug)]
pub(crate) enum OutcomeKind {
    Possible,
    Impossible,
}

impl OutcomeKind {
    pub(super) fn new(s: impl AsRef<str>) -> Self {
        info!("raw outcome: {}", s.as_ref());

        match s.as_ref() {
            "Possible" => OutcomeKind::Possible,
            "Impossible" => OutcomeKind::Impossible,
            _ => panic!("cmd output was not among possible outcomes"),
        }
    }
}
