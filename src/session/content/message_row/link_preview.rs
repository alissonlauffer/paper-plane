use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use tdlib::enums::MessageContent;

use crate::session::content::message_row::MessageBase;
use crate::tdlib::Message;

mod imp {
    use once_cell::sync::Lazy;

    use crate::session::content::message_row::base::MessageBaseImpl;

    use super::*;
    use std::cell::RefCell;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/melix99/telegrand/ui/content-message-link-preview.ui")]
    pub(crate) struct LinkPreview {
        pub(super) bindings: RefCell<Vec<gtk::ExpressionWatch>>,
        pub(super) message: RefCell<Option<Message>>,
        #[template_child]
        pub(super) name_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) title_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) content_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) separator: TemplateChild<gtk::Separator>,
        #[template_child]
        pub(super) photo_thumbnail: TemplateChild<gtk::Picture>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LinkPreview {
        const NAME: &'static str = "ContentMessageLinkPreview";
        type Type = super::LinkPreview;
        type ParentType = MessageBase;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LinkPreview {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![glib::ParamSpecObject::new(
                    "message",
                    "Message",
                    "The message represented by this row",
                    Message::static_type(),
                    glib::ParamFlags::READWRITE | glib::ParamFlags::EXPLICIT_NOTIFY,
                )]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            match pspec.name() {
                "message" => obj.set_message(value.get().unwrap()),
                _ => unimplemented!(),
            }
        }

        fn property(&self, obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "message" => obj.message().to_value(),
                _ => unimplemented!(),
            }
        }
    }

    impl WidgetImpl for LinkPreview {}
    impl MessageBaseImpl for LinkPreview {}
}

glib::wrapper! {
    pub(crate) struct LinkPreview(ObjectSubclass<imp::LinkPreview>)
        @extends gtk::Widget;
}

impl LinkPreview {
    pub(crate) fn message(&self) -> Message {
        self.imp().message.borrow().clone().unwrap()
    }

    pub(crate) fn set_message(&self, message: &Message) {
        let imp = self.imp();

        if imp.message.borrow().as_ref() == Some(message) {
            return;
        }

        if let Some(message) = message.downcast_ref::<Message>() {
            if let MessageContent::MessageText(content) = message.content().0 {
                if let Some(web_page) = content.web_page {
                    imp.name_label.set_text(&web_page.site_name);
                    imp.title_label.set_text(&web_page.title);
                    imp.content_label.set_text(&web_page.description.text);
                }
            }
        }

        let mut bindings = imp.bindings.borrow_mut();
        while let Some(binding) = bindings.pop() {
            binding.unwatch();
        }
    }
}
