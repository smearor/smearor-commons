//! GTK widget glue: factories that hand draw callbacks a typed
//! [`Size`] instead of two raw extents.

mod drawing_area;

pub use drawing_area::drawing_area;
pub use drawing_area::drawing_target;
