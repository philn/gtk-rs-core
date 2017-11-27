// This file was generated by gir (d50d839) from gir-files (469db10)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Dialog;
use RecentChooser;
use Widget;
use Window;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct RecentChooserDialog(Object<ffi::GtkRecentChooserDialog, ffi::GtkRecentChooserDialogClass>): Dialog, Window, Bin, Container, Widget, Buildable, RecentChooser;

    match fn {
        get_type => || ffi::gtk_recent_chooser_dialog_get_type(),
    }
}

impl RecentChooserDialog {
    //pub fn new<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: IsA<Window> + 'b, R: Into<Option<&'b Q>>, S: Into<Option<&'c str>>>(title: P, parent: R, first_button_text: S, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> RecentChooserDialog {
    //    unsafe { TODO: call ffi::gtk_recent_chooser_dialog_new() }
    //}

    //pub fn new_for_manager<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: IsA<Window> + 'b, R: Into<Option<&'b Q>>, S: Into<Option<&'c str>>>(title: P, parent: R, manager: &RecentManager, first_button_text: S, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> RecentChooserDialog {
    //    unsafe { TODO: call ffi::gtk_recent_chooser_dialog_new_for_manager() }
    //}
}
