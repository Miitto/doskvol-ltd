#[proc_macro]
pub fn blades(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let playbook_str = include_str!("../blades/playbook.json");

    let playbook: PlaybookWrapper =
        serde_json::from_str(playbook_str).expect("Failed to parse playbook JSON");

    let playbook = playbook.playbook;

    let playbook_count = playbook.len();

    let class_items_str = include_str!("../blades/class_items.json");
    let class_items: ClassItems =
        serde_json::from_str(class_items_str).expect("Failed to parse class items JSON");

    quote::quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        pub enum Class {
            Cutter,
            Hound,
            Leech,
            Lurk,
            Slide,
            Spider,
            Whisper
        }

        impl std::fmt::Display for Class {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Class::Cutter => write!(f, "Cutter"),
                    Class::Hound => write!(f, "Hound"),
                    Class::Leech => write!(f, "Leech"),
                    Class::Lurk => write!(f, "Lurk"),
                    Class::Slide => write!(f, "Slide"),
                    Class::Spider => write!(f, "Spider"),
                    Class::Whisper => write!(f, "Whisper"),
                }
            }
        }

        pub mod playbook {
            use super::Class;

            #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
            pub struct Ability {
                pub name: &'static str,
                pub class: Class,
                pub description: super::Description<&'static str>,
            }

            pub const PLAYBOOK: [Ability; #playbook_count] = [
                #(
                    #playbook
                ),*
            ];
        }

        pub mod items {
            #class_items
        }
    }
    .into()
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum Class {
    Cutter,
    Hound,
    Leech,
    Lurk,
    Slide,
    Spider,
    Whisper,
}

impl quote::ToTokens for Class {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let string = format!("{self:?}");
        let ident = quote::format_ident!("{}", string);

        tokens.extend(quote::quote! {
            Class::#ident
        });
    }
}

#[derive(Debug, serde::Deserialize)]
struct PlaybookWrapper {
    playbook: Vec<PlaybookAbility>,
}

#[derive(Debug, serde::Deserialize)]
struct PlaybookAbility {
    name: String,
    class: Class,
    description: String,
}

impl quote::ToTokens for PlaybookAbility {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let class = &self.class;
        let description = &self.description;

        tokens.extend(quote::quote! {
            Ability {
                name: #name,
                class: #class,
                description: super::Description::new(#description),
            }
        });
    }
}

#[derive(Debug, serde::Deserialize)]
struct ClassItems {
    cutter: Vec<String>,
    hound: Vec<String>,
    leech: Vec<String>,
    lurk: Vec<String>,
    slide: Vec<String>,
    spider: Vec<String>,
    whisper: Vec<String>,
}

impl quote::ToTokens for ClassItems {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let cutter = &self.cutter;
        let hound = &self.hound;
        let leech = &self.leech;
        let lurk = &self.lurk;
        let slide = &self.slide;
        let spider = &self.spider;
        let whisper = &self.whisper;

        let cutter_count = cutter.len();
        let hound_count = hound.len();
        let leech_count = leech.len();
        let lurk_count = lurk.len();
        let slide_count = slide.len();
        let spider_count = spider.len();
        let whisper_count = whisper.len();

        tokens.extend(quote::quote! {
        #[derive(Debug)]
        struct ClassItems {
            pub cutter: [super::Description<&'static str>; #cutter_count],
            pub hound: [super::Description<&'static str>; #hound_count],
            pub leech: [super::Description<&'static str>; #leech_count],
            pub lurk: [super::Description<&'static str>; #lurk_count],
            pub slide: [super::Description<&'static str>; #slide_count],
            pub spider: [super::Description<&'static str>; #spider_count],
            pub whisper: [super::Description<&'static str>; #whisper_count],
        }

        const CLASS_ITEMS: ClassItems = ClassItems {
            cutter: [#(
                        super::Description::new(#cutter)
                        ),*],
            hound: [#(
                        super::Description::new(#hound)
                        ),*],
            leech: [#(
                        super::Description::new(#leech)
                        ),*],
            lurk: [#(
                        super::Description::new(#lurk)
                        ),*],
            slide: [#(
                        super::Description::new(#slide)
                        ),*],
            spider: [#(
                        super::Description::new(#spider)
                        ),*],
            whisper: [#(
                        super::Description::new(#whisper)
                        ),*],
        };
        });
    }
}
