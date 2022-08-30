pub struct Paths<'a> {
    pub create: &'a str,
    pub join: &'a str,
    pub stream: &'a str,
}

pub static PATHS: Paths = {
    let create = "create";
    let join = "join";
    let stream = "stream";
    Paths {
        create,
        join,
        stream,
    }
};
