use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use shumate::traits::LocationExt;
use shumate::traits::MarkerExt;

use crate::ui;

// The golden ratio
const PHI: f64 = 1.6180339887;
const MIN_HEIGHT: i32 = 150;
const MAX_HEIGHT: i32 = 225;
const MIN_WIDTH: i32 = (MIN_HEIGHT as f64 * PHI) as i32;
const MAX_WIDTH: i32 = (MAX_HEIGHT as f64 * PHI) as i32;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/app/drey/paper-plane/ui/components/map.ui")]
    pub(crate) struct Map {
        pub(super) marker: shumate::Marker,
        #[template_child]
        pub(super) marker_image: TemplateChild<gtk::Image>,
        #[template_child]
        pub(super) map: TemplateChild<shumate::Map>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Map {
        const NAME: &'static str = "PaplMap";
        type Type = super::Map;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            klass.set_css_name("map");
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Map {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = &*self.obj();

            obj.set_custom_marker(None);

            let map_source = shumate::MapSourceRegistry::with_defaults()
                .by_id(shumate::MAP_SOURCE_OSM_MAPNIK)
                .unwrap();
            self.map.set_map_source(&map_source);

            let viewport = self.map.viewport().unwrap();

            let map_layer = shumate::MapLayer::new(&map_source, &viewport);
            self.map.add_layer(&map_layer);

            let marker_layer = shumate::MarkerLayer::new(&viewport);
            marker_layer.add_marker(&self.marker);
            self.map.add_layer(&marker_layer);
        }

        fn dispose(&self) {
            let mut child = self.obj().first_child();
            while let Some(child_) = child {
                child = child_.next_sibling();
                child_.unparent();
            }
        }
    }

    impl WidgetImpl for Map {
        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            let widget = &*self.obj();
            let (min_size, natural_size) = match orientation {
                gtk::Orientation::Horizontal => widget.measure(for_size, MIN_WIDTH, MAX_WIDTH),
                _ => widget.measure((for_size as f64 / PHI) as i32, MIN_HEIGHT, MAX_HEIGHT),
            };

            (min_size, natural_size, -1, -1)
        }

        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            let mut child = self.obj().first_child();
            while let Some(child_) = child {
                child = child_.next_sibling();
                child_.allocate(width, height, baseline, None);
            }
        }
    }
}

glib::wrapper! {
    pub(crate) struct Map(ObjectSubclass<imp::Map>)
        @extends gtk::Widget;
}

impl Map {
    fn measure(&self, for_size: i32, min_size: i32, max_size: i32) -> (i32, i32) {
        let natural_size = if for_size == -1 {
            max_size
        } else {
            self.measure_children(gtk::Orientation::Horizontal, for_size);
            for_size.min(max_size).max(min_size)
        };
        (min_size, natural_size)
    }

    fn measure_children(&self, orientation: gtk::Orientation, for_size: i32) {
        let mut child = self.first_child();
        while let Some(child_) = child {
            child = child_.next_sibling();
            child_.measure(orientation, for_size);
        }
    }

    pub(crate) fn set_custom_marker(&self, marker: Option<gtk::Widget>) {
        let imp = self.imp();
        imp.marker.set_child(Some(&ui::MapMarker::from(
            &marker.unwrap_or_else(|| imp.marker_image.get().upcast()),
        )));
    }

    pub(crate) fn set_marker_position(&self, lat: f64, lon: f64) {
        self.imp().marker.set_location(lat, lon);
    }

    fn center_location(&self, lat: f64, lon: f64) {
        let viewport = self.imp().map.viewport().unwrap();
        viewport.set_zoom_level(16.0);
        viewport.set_location(lat, lon);
    }

    pub(crate) fn center_marker(&self) {
        let imp = self.imp();
        self.center_location(imp.marker.latitude(), imp.marker.longitude());
    }
}
