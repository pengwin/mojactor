use std::any::Any;

/// Unwinds panic to Result<.., String>
///
/// # Errors
///
/// Returns panic content as Err(String)
pub fn unwind_panic<R>(result: Result<R, Box<dyn Any + Send>>) -> Result<R, String> {
    match result {
        Ok(r) => Ok(r),
        Err(e) => {
            if let Some(s) = e.downcast_ref::<&str>() {
                Err((*s).to_string())
            } else if let Some(s) = e.downcast_ref::<String>() {
                Err(s.clone())
            } else {
                Err(format!("Unknown panic {e:?}"))
            }
        }
    }
}
