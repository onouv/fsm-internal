
# FSM-INTERNAL

Example for a finite state machine internal to some entity, where during the transitions some business logic would generate different Results. The `next_state()` method returns `None` as an inidication that no transition is defined.

```
$ fsm-internal ±|main ✗|→ cargo run
Initial state: S0
(State::S0, Event::E2) => (S0, None)
(State::S0, Event::E1) => (S1, Some(A(A)))
(State::S1, Event::E2) => (S2, Some(B(B)))
(State::S2, Event::E2) => (S2, None)
(State::S2, Event::E1) => (S3, Some(A(A)))
```