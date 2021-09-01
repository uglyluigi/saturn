pub mod google_login_button;
pub mod home;
pub mod notfound;
pub mod pg_login;
pub mod stellar;
pub mod toolbar;
pub mod router;
pub mod club_view;

#[macro_use]
pub mod mac {
    #[macro_export]
    macro_rules! tell {
        ($str_slice:expr) => (
            web_sys::console::log_1(&$str_slice.into())
        );
    
        ($str_slice:expr, $($arg:expr),*) => (
            tell!(format!($str_slice, $($arg),*))
        )
    }
}