use cosmwasm_std::Event;
use cw_multi_test::AppResponse;

pub fn given_contains_expected(given: &Event, expected: &Event) -> bool {
    expected.ty == given.ty
        && expected
            .attributes
            .iter()
            .all(|at| given.attributes.contains(at))
}

// To bypass `attribute key `{key}` is invalid - keys starting with an underscore are reserved` panic in Attribute::new
pub fn new_event(ty: impl Into<String>, attributes: Vec<(&str, &str)>) -> Event {
    let mut event = Event::new(ty);
    for (key, value) in attributes {
        event = event.add_attribute(key, value);
    }
    event
}

pub fn assert_strict_event_attributes(
    res: AppResponse,
    expected_ty: impl Into<String>,
    expected_attributes: Vec<(&str, &str)>,
) {
    let expected_event = &new_event(expected_ty, expected_attributes);
    assert!(
        res.events.iter().any(|given| {
            given_contains_expected(given, expected_event)
                && given_contains_expected(expected_event, given)
        }),
        "Expected to find an event {:?}, but received: {:?}",
        expected_event,
        res.events
    );
}
