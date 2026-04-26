use rocksalt_shared::event::{ Detail, command_for_webview };
use serde::Serialize;

pub fn dispatch<D>(detail: D) -> String
    where D: Detail + Serialize
{
    command_for_webview(detail)
}
