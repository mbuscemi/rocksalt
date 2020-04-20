pub fn on_some<T, F>(option: Option<T>, f: F) -> ()
    where F: FnOnce(T) -> ()
{
    match option {
        Some(x) => { f(x); () },
        None => (),
    }
}
