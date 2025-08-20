#[proc_macro]
pub fn blades(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let playbook_str = include_str!("../blades/playbook.json");

    let playbook: PlaybookWrapper =
        serde_json::from_str(playbook_str).expect("Failed to parse playbook JSON");

    let playbook = playbook.playbook;

    let count = playbook.len();

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

            pub const PLAYBOOK: [Ability; #count] = [
                #(
                    #playbook
                ),*
            ];
        }
    }
    .into()
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
