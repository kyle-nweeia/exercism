use time::PrimitiveDateTime;

pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    start.saturating_add(time::Duration::seconds(1_000_000_000))
}
