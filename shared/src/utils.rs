pub fn on_some<T, F>(option: Option<T>, f: F) -> ()
    where F: FnOnce(T) -> ()
{
    match option {
        Some(x) => { f(x); () },
        None => (),
    }
}

pub fn on_ok<T, E, F>(result: Result<T, E>, f: F) -> ()
    where F: FnOnce(T) -> ()
{
    match result {
        Ok(x) => { f(x); () },
        Err(_) => ()
    }
}
