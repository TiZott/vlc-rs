// Copyright (c) 2018 Tino Zottmann
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use sys;
use ::{Instance, MediaList};
use ::std::ptr;
use ::tools::{to_cstr, from_cstr};

/// Category of a media discoverer.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MediaDiscovererCategory {
    /// Devices, like portable music player.
    Devices = 0,
    /// LAN/WAN services, like Upnp, SMB, or SAP.
    LAN,
    /// Podcasts.
    Podcasts,
    /// Local directories, like Video, Music or Pictures directories.
    LocalDirs
}

/// Media discoverer description
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct MediaDiscovererDescription {
    pub name : String,
    pub long_name : String,
    pub cat : MediaDiscovererCategory
}

/// LibVLC media discovery finds available media via various means.
pub struct MediaDiscoverer {
    pub(crate) ptr: *mut sys::libvlc_media_discoverer_t,
}

impl MediaDiscoverer {
    /// Create a media discoverer object by name.
    /// After this object is created, you should attach to media_list events in order to be notified of new items discovered.
    /// You need to call start() in order to start the discovery.
    pub fn new( instance: &Instance, name: &str ) -> Option< MediaDiscoverer > {
        let cstr = to_cstr( name );

        unsafe {
            let p = sys::libvlc_media_discoverer_new( instance.ptr, cstr.as_ptr() );
            if p.is_null() { None } else { Some( MediaDiscoverer { ptr: p } ) }
        }
    }

    /// Start media discovery.
    /// To stop it, callstop() or list_release() directly.
    pub fn start( &self ) -> Result< (), () > {
        unsafe { if sys::libvlc_media_discoverer_start( self.ptr ) == 0 { Ok(()) } else { Err(()) } }
    }

    /// Stop media discovery.
    pub fn stop( &self ) {
        unsafe { sys::libvlc_media_discoverer_stop( self.ptr ); }
    }

    /// Get media service discover media list.
    pub fn media_list( &self ) -> Option< MediaList > {
        unsafe {
            let ptr = sys::libvlc_media_discoverer_media_list( self.ptr );
            if ptr.is_null() { None } else { Some( MediaList { ptr } ) }
        }
    }

    /// Query if media service discover object is running.
    pub fn is_running( &self ) -> bool {
        unsafe { if sys::libvlc_media_discoverer_is_running( self.ptr ) == 0 { false } else { true } }
    }

    /// Get media discoverer services by category.
    pub fn list_get( instance: &Instance, category: MediaDiscovererCategory ) -> Result< Vec< MediaDiscovererDescription >, () > {
        unsafe{
            let i_cat = match category {
                MediaDiscovererCategory::Devices => sys::libvlc_media_discoverer_category_t::libvlc_media_discoverer_devices,
                MediaDiscovererCategory::LAN => sys::libvlc_media_discoverer_category_t::libvlc_media_discoverer_lan,
                MediaDiscovererCategory::Podcasts => sys::libvlc_media_discoverer_category_t::libvlc_media_discoverer_podcasts,
                MediaDiscovererCategory::LocalDirs => sys::libvlc_media_discoverer_category_t::libvlc_media_discoverer_localdirs,
            };
            let mut list : *mut *mut sys::libvlc_media_discoverer_description_t = ptr::null_mut();
            let count = sys::libvlc_media_discoverer_list_get( instance.ptr, i_cat, &mut list );
            if count == 0 {
                return Err(())
            }
            let mut array = Vec::new();
            for index in 0 .. count {
                let item = **list.offset( index as isize );
                array.push( MediaDiscovererDescription {
                    name : from_cstr( item.psz_name ).unwrap_or_default(),
                    long_name : from_cstr( item.psz_longname ).unwrap_or_default(),
                    cat :  match item.i_cat {
                        sys::libvlc_media_discoverer_category_t::libvlc_media_discoverer_devices => MediaDiscovererCategory::Devices,
                        sys::libvlc_media_discoverer_category_t::libvlc_media_discoverer_lan => MediaDiscovererCategory::LAN,
                        sys::libvlc_media_discoverer_category_t::libvlc_media_discoverer_podcasts => MediaDiscovererCategory::Podcasts,
                        sys::libvlc_media_discoverer_category_t::libvlc_media_discoverer_localdirs => MediaDiscovererCategory::LocalDirs,
                    }
                } );
            }
            sys::libvlc_media_discoverer_list_release( list, count );

            return Ok( array )
        }
    }

    /// Returns raw pointer
    pub fn raw(&self) -> *mut sys::libvlc_media_discoverer_t {
        self.ptr
    }
}

impl Drop for MediaDiscoverer {
    fn drop( &mut self ) {
        unsafe{ sys::libvlc_media_discoverer_release( self.ptr ) };
    }
}
