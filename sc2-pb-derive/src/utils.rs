
use super::{syn, quote};

/// convert a field name like `FooThing` into a name like `set_foo_thing`
pub fn construct_field_accessor<T: AsRef<str>>(t: T, prefix: &str) -> quote::Ident {
    let s: &str = t.as_ref();
    let mut r = String::with_capacity(s.len());
    r.push_str(prefix);
    let mut first = true;
    for c in s.chars() {
        if c.is_uppercase() {
            r.push('_');
            r.push(c.to_ascii_lowercase());
        } else {
            if first {
                r.push('_')
            }
            r.push(c);
        }
        first = false;
    }
    quote::Ident::from(r)
}

pub fn get_type_ident(ty: &syn::Ty) -> quote::Ident {
    if let &syn::Ty::Path(_,
                          syn::Path {
                              ref global,
                              ref segments,
                          }) = ty
        {
            if segments.len() > 0 {
                return quote::Ident::from(segments[0].ident.as_ref());
            }
        }
    panic!("Can't extract ident from ty")
}

pub fn get_printable_ty(ty: &syn::Ty) -> String {
    use quote::ToTokens;

    let mut t = quote::Tokens::new();
    ty.to_tokens(&mut t);


    format!("{}", t)
}

pub fn is_option(ty: &syn::Ty) -> bool {
    if let &syn::Ty::Path(_,
                          syn::Path {
                              ref global,
                              ref segments,
                          }) = ty
        {
            // check to see if the first path segment is an Option
            segments.len() > 0 && segments[0].ident.as_ref() == "Option"
        } else {
        false
    }
}

/// Tries to determine if a given type (like Option<protos::Response>) references
/// a protobuf type somewhere.  This works by hard-coding the fact the protobuffer type
/// live in a module named "protos"
pub fn is_protobuf_type(ty: &syn::Ty) -> bool {
    if let &syn::Ty::Path(_, syn::Path { ref global, ref segments }) = ty {
        segments.iter().any(|segment| {
            segment.ident.as_ref() == "protos" || match &segment.parameters {
                &syn::PathParameters::AngleBracketed(ref data) => {
                    data.types.iter().any(|t| is_protobuf_type(t))
                }
                &syn::PathParameters::Parenthesized(ref data) => {
                    data.inputs.iter().any(|t| is_protobuf_type(t))
                }
            }
        })
    } else {
        false
    }
}

pub fn is_equal(ty: &syn::Ty, s: &str) -> bool {
    //ty == &syn::Ty::Path(None, syn::parse_path(s).unwrap())
    ty == &syn::parse_type(s).unwrap()
}

pub fn get_attr(attrs: &[syn::Attribute], name: &str) -> Option<String> {
    let x = attrs
        .iter()
        .filter_map(|attr| {
            if let syn::MetaItem::NameValue(ref id, syn::Lit::Str(ref s, _)) = attr.value {
                if id.as_ref() == name {
                    return Some(s.clone());
                }
            }
            if let syn::MetaItem::List(ref id, ref vec) = attr.value {
                if id.as_ref() == name {
                    if vec.len() > 0 {
                        match &vec[0] {
                            &syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ref ident)) => {
                                return Some(ident.as_ref().to_owned());
                            }
                            &syn::NestedMetaItem::Literal(syn::Lit::Str(ref s, _)) => {
                                return Some(s.to_owned());
                            }
                            _ => {}
                        }
                    }
                }
            }
            if let syn::MetaItem::Word(ref id) = attr.value {
                if id.as_ref() == name {
                    return Some(String::new());
                }
            }
            None
        })
        .next();
    x
}
