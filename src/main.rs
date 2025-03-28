
/// Business operation results, here modeled as empty structs
#[derive(Debug)]
struct A;
#[derive(Debug)]
struct B;
#[derive(Debug)]
struct C;

/// Return type for the FSM transition function,
/// cavering all possible different results from the 
/// business operations  
#[derive(Debug)]
enum Result {
    A(A),
    B(B),
    C(C)
}

#[derive(Debug, Copy, Clone)]
enum State {
    S0,
    S1,
    S2,
    S3
}

enum Event {
    E1,
    E2
}

/// The finite state machine
#[derive(Debug)]
struct Stateful {
    state: State
}
impl Stateful {

    /// initial default state 
    pub fn new() -> Self {
        Self {
            state: State::S0
        }
    }
    
    pub fn get_state(&self) -> State {
        self.state
    }

    /// The fsm transition function. All possible transitions are 
    /// modeled here. Also, all guard logic must be modelled here. 
    /// Business methods must be called from here and the results must be 
    /// wrapped in the common return type.
    /// Implements the following transitions: 
    /// S0 -E1-> S1(A) -E2-> S2(B) -E1-> S3(C)
    ///          S1    -E1->             S3(A)
    /// all other transitions are suppressed and deliver 'None'.

    pub fn next_state(&mut self, event: Event) -> Option<Result> {
        match (self.state, event) {
            (State::S0, Event::E1) => {
                self.state = State::S1;
                return Some(Result::A(self.do_business()));
            }
            (State::S1, Event::E1) => {
                self.state = State::S3;
                return Some(Result::C(C{}));
            },
            (State::S1, Event::E2) => {
                self.state = State::S2;
                return Some(Result::B(B{}));
            },
            (State::S2, Event::E1) => {
                self.state = State::S3;
                return Some(Result::A(A{}));
            },
            _ => None
        }        type 
    }

    /// Business methods ...
    fn do_business(&self) -> A {
        return A {};
    }
}

fn main() {
    let mut fsm = Stateful::new();
    println!("Initial state: {:?}", fsm.get_state());
    

    let result = fsm.next_state(Event::E2);
    println!("(State::S0, Event::E2) => ({:?}, {:?})", fsm.get_state(), result);
    
    let result = fsm.next_state(Event::E1);
    println!("(State::S0, Event::E1) => ({:?}, {:?})", fsm.get_state(), result);
    
    let result = fsm.next_state(Event::E2);
    println!("(State::S1, Event::E2) => ({:?}, {:?})", fsm.get_state(), result);

    let result = fsm.next_state(Event::E2);
    println!("(State::S2, Event::E2) => ({:?}, {:?})", fsm.get_state(), result);

    let result = fsm.next_state(Event::E1);
    println!("(State::S2, Event::E1) => ({:?}, {:?})", fsm.get_state(), result);
    
}