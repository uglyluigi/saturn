pub mod google_login_button;
pub mod home;
pub mod notfound;
pub mod pg_login;
pub mod stellar;
pub mod toolbar;
pub mod router;
pub mod club_view;
pub mod club_card;
pub mod three;

#[macro_use]
pub mod mac {
    /**
     * Tell is a macro that uses web_sys to print text to the console.
     * You can use it exactly the same as format!.
     */
    #[macro_export]
    macro_rules! tell {
        ($str_slice:expr) => (
            web_sys::console::log_1(&$str_slice.into())
        );
    
        ($str_slice:expr, $($arg:expr),*) => (
            tell!(format!($str_slice, $($arg),*))
        )
    }

    /**
     * Please is a macro that simplifies a ridiculously long expression found when
     * trying to put the value inside of an Option<Option<String>> in the DOM.
     * 
     * First, the Option stored in the component's state must be borrowed (as_ref) then
     * unwrapped so it isn't moved into the view() function and dropped. The same thing happens
     * with the Option inside that Option. Then the String is cloned.
     * 
     * This is a good example of how Rust's enum and memory systems can make things a little crazy,
     * but how basically any example of this kind of thing can be overcome easily with macros.
     */
    #[macro_export]
    macro_rules! please {
        ($thing:expr, $prop:ident) => {
            std::string::String::from($thing.as_ref().unwrap().$prop.as_ref().unwrap().clone())
        };
    }
}