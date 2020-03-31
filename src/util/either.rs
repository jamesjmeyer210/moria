pub enum Either <A,B> {
    This(A),
    That(B),
    None,
}

impl <A,B>Either<A,B> {

    pub fn this(self) -> Option<A> {
        match self {
            Either::This(a) => Some(a),
            _ => None,
        }

    }

    pub fn that(self) -> Option<B> {
        match self {
            Either::That(b) => Some(b),
            _ => None
        }
    }

    pub fn has_any(self) -> bool {
        match self {
            Either::This(a) => true,
            Either::That(b) => true,
            Neither => false
        }
    }

    pub fn has_none(self) -> bool {
        !self.has_any()
    }
}