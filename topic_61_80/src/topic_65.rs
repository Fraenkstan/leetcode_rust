pub fn is_number(s: String) -> bool {
    s.chars()
        .try_fold(State::new(), State::handle)
        .as_ref()
        .map_or(false, State::is_valid)
}

type Result = std::result::Result<State, ()>;

enum State {
    Start,
    Sign,
    Integer,
    Dot,
    EmptyDot,
    Decimal,
    E,
    ExpSign,
    Exponent,
    End,
}

impl State {
    pub fn new() -> Self {
        State::Start
    }

    pub fn is_valid(&self) -> bool {
        use State::*;
        match self {
            Start | Sign | E | ExpSign | EmptyDot => false,
            _ => true,
        }
    }

    pub fn handle(self, c: char) -> Result {
        use State::*;
        match self {
            Start => match c {
                ' ' => Ok(Start),
                '+' | '-' => Ok(Sign),
                '0'..='9' => Ok(Integer),
                '.' => Ok(EmptyDot),
                _ => Err(()),
            },
            Sign => match c {
                '0'..='9' => Ok(Integer),
                '.' => Ok(EmptyDot),
                _ => Err(()),
            },
            Integer => match c {
                '0'..='9' => Ok(Integer),
                '.' => Ok(Dot),
                'e' | 'E' => Ok(E),
                ' ' => Ok(End),
                _ => Err(()),
            },
            EmptyDot => match c {
                '0'..='9' => Ok(Decimal), // "  .1" or "  +.1"
                _ => Err(()),
            },
            Dot => match c {
                '0'..='9' => Ok(Decimal),
                'e' => Ok(E), // "46.e3"
                ' ' => Ok(End),
                _ => Err(()),
            },
            Decimal => match c {
                '0'..='9' => Ok(Decimal),
                'e' => Ok(E),
                ' ' => Ok(End),
                _ => Err(()),
            },
            E => match c {
                '+' | '-' => Ok(ExpSign),
                '0'..='9' => Ok(Exponent),
                _ => Err(()),
            },
            ExpSign => match c {
                '0'..='9' => Ok(Exponent),
                _ => Err(()),
            },
            Exponent => match c {
                '0'..='9' => Ok(Exponent),
                ' ' => Ok(End),
                _ => Err(()),
            },
            End => match c {
                ' ' => Ok(End),
                _ => Err(()),
            },
        }
    }
}
