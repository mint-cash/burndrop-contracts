use cosmwasm_std::Event;
use cw_multi_test::AppResponse;

pub fn given_contains_expected(given: &Event, expected: &Event) -> bool {
    expected.ty == given.ty
        && expected
            .attributes
            .iter()
            .all(|at| given.attributes.contains(at))
}

pub fn assert_event_strict(res: AppResponse, expected: &Event) {
    assert!(
        res.events.iter().any(|given| {
            given_contains_expected(given, expected) && given_contains_expected(expected, given)
        }),
        "Expected to find an event {:?}, but received: {:?}",
        expected,
        res.events
    );
}
