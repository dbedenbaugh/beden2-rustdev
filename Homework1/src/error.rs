mod error {
    //use axum::Reject;
    #[derive(Debug)]
    enum Error {
        ParseError(std::num::ParseIntError),
        MissingParameters,
        QuestionNotFound,
    }
 
    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
                Error::ParseError(ref err) => {
                    write!(f, "Cannot parse parameter: {}", err)
                },
                Error::MissingParameters => write!(f, "Missing parameter"),
                Error::QuestionNotFound => write!(f, "Question not found"),
            }
        }
    }
 
    //impl Reject for Error {}
 
}