#![allow(
    dead_code,
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes
)]

extern crate cairo;
extern crate cairo_sys;
extern crate gio_sys;
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gtypes;

mod dep_types {
    pub use cairo_sys::{cairo_region_t, cairo_surface_t, cairo_t};
    pub use glib_sys::{
        gboolean, gpointer, GArray, GDate, GError, GList, GQuark, GString, GTime, GType, GTree, GBytes
    };
    pub use gobject_sys::{GObject, GObjectClass};
    pub use gtypes::{gchar, gdouble, gint, gsize, guint, gushort};
    pub type guint64 = u64;
    pub use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort};
    pub type time_t = c_long;
    pub use gio_sys::{GCancellable, GFile, GInputStream};
}

pub mod poppler {
    use super::dep_types::*;
    use super::poppler_action::{_PopplerAction, _PopplerActionLayer, _PopplerDest};
    use super::poppler_annot::_PopplerAnnotCalloutLine;
    use super::poppler_attachment::_PopplerAttachment;
    use super::poppler_movie::_PopplerMovie;
    use super::poppler_page::{
        _PopplerAnnotMapping, _PopplerColor, _PopplerFormFieldMapping, _PopplerImageMapping,
        _PopplerLinkMapping, _PopplerPageTransition, _PopplerPoint, _PopplerQuadrilateral,
        _PopplerRectangle, _PopplerTextAttributes,
    };
    include!(concat!(env!("OUT_DIR"), "/bindings_poppler.rs"));
}

pub mod poppler_document {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_document.rs"));
}

pub mod poppler_page {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_page.rs"));
}

pub mod poppler_action {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_action.rs"));
}

pub mod poppler_annot {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_annot.rs"));
}

pub mod poppler_attachment {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_attachment.rs"));
}

pub mod poppler_form_field {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_form_field.rs"));
}

pub mod poppler_layer {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_layer.rs"));
}

pub mod poppler_media {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_media.rs"));
}

pub mod poppler_movie {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_movie.rs"));
}
