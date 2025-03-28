#[derive(Debug)]
struct A;
#[derive(Debug)]
struct B;
#[derive(Debug)]
struct C;


enum Event {
    E1,
    E2
}

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

#[derive(Debug)]
struct Stateful {
    state: State
}
impl Stateful {
    pub fn new() -> Self {
        Self {
            state: State::S0
        }
    }
    
    pub fn get_state(&self) -> State {
        self.state
    }

    // S0 -E1-> S1(A) -E2-> S2(B) -E1-> S3(C)
    //          S1    -E1->             S3(A)
    pub fn next_state(&mut self, event: Event) -> Option<Result> {
        match (self.state, event) {
            (State::S0, Event::E1) => {
                
                self.state = State::S1;
                return Some(Result::A(A{}));
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
        }        
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