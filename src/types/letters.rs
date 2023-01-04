#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LetterState {
    Initial,
    Correct,
    Present,
    Absent,
}
