use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute, DeriveInput, Expr, FnArg, ItemFn, Lifetime, Lit, Meta, Pat, ReturnType, Type,
    parse_macro_input,
};

/// Information about a single prop extracted from the function signature
struct PropInfo {
    name: syn::Ident,
    ty: Type,
    ty_string: String,
    is_required: bool,
    default_value: Option<Expr>,
    default_string: Option<String>,
    doc_comments: Vec<Attribute>,
    doc_string: Option<String>,
}

/// Check if a type contains any references (with or without explicit lifetimes)
fn type_has_references(ty: &Type) -> bool {
    match ty {
        Type::Reference(_) => true,
        Type::Path(type_path) => {
            for segment in &type_path.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        if let syn::GenericArgument::Type(inner_ty) = arg {
                            if type_has_references(inner_ty) {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        }
        Type::Tuple(type_tuple) => type_tuple.elems.iter().any(type_has_references),
        Type::Slice(type_slice) => type_has_references(&type_slice.elem),
        Type::Array(type_array) => type_has_references(&type_array.elem),
        _ => false,
    }
}

/// Add lifetime 'a to all references in a type that don't have explicit lifetimes
fn add_lifetime_to_references(ty: &Type, lifetime: &Lifetime) -> Type {
    match ty {
        Type::Reference(type_ref) => {
            let mut new_ref = type_ref.clone();
            if new_ref.lifetime.is_none() {
                new_ref.lifetime = Some(lifetime.clone());
            }
            new_ref.elem = Box::new(add_lifetime_to_references(&type_ref.elem, lifetime));
            Type::Reference(new_ref)
        }
        Type::Path(type_path) => {
            let mut new_path = type_path.clone();
            for segment in &mut new_path.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &mut segment.arguments {
                    for arg in &mut args.args {
                        if let syn::GenericArgument::Type(inner_ty) = arg {
                            *inner_ty = add_lifetime_to_references(inner_ty, lifetime);
                        }
                    }
                }
            }
            Type::Path(new_path)
        }
        Type::Tuple(type_tuple) => {
            let mut new_tuple = type_tuple.clone();
            for elem in &mut new_tuple.elems {
                *elem = add_lifetime_to_references(elem, lifetime);
            }
            Type::Tuple(new_tuple)
        }
        Type::Slice(type_slice) => {
            let mut new_slice = type_slice.clone();
            new_slice.elem = Box::new(add_lifetime_to_references(&type_slice.elem, lifetime));
            Type::Slice(new_slice)
        }
        Type::Array(type_array) => {
            let mut new_array = type_array.clone();
            new_array.elem = Box::new(add_lifetime_to_references(&type_array.elem, lifetime));
            Type::Array(new_array)
        }
        _ => ty.clone(),
    }
}

/// Parse #[prop(...)] attributes from a function parameter
fn parse_prop_attr(attrs: &[Attribute]) -> (bool, Option<Expr>) {
    for attr in attrs {
        if attr.path().is_ident("prop") {
            // Parse #[prop(default = value)] or #[prop(default)]
            let mut default_value = None;
            let mut has_default = false;

            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("default") {
                    has_default = true;
                    // Check if there's a value: #[prop(default = X)]
                    if meta.input.peek(syn::Token![=]) {
                        let _: syn::Token![=] = meta.input.parse()?;
                        let value: Expr = meta.input.parse()?;
                        default_value = Some(value);
                    }
                    // Otherwise it's just #[prop(default)] - use Default::default()
                }
                Ok(())
            });

            if has_default {
                return (false, default_value); // Not required, has default
            }
        }
    }
    (true, None) // Required by default, no default value
}

/// Extract doc comments from attributes
fn extract_doc_comments(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .cloned()
        .collect()
}

/// Extract doc comments as a single string (for registry metadata)
fn extract_doc_string(attrs: &[Attribute]) -> Option<String> {
    let docs: Vec<String> = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            if let Meta::NameValue(meta) = &attr.meta {
                if let Expr::Lit(expr_lit) = &meta.value {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        return Some(lit_str.value().trim().to_string());
                    }
                }
            }
            None
        })
        .collect();

    if docs.is_empty() {
        None
    } else {
        Some(docs.join(" "))
    }
}

/// Check if a type is Option<T>
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// Transforms a function into a component struct with builder pattern API.
///
/// # Example
///
/// ```rust,ignore
/// #[component]
/// pub fn Button(
///     /// The button text (required)
///     label: &str,
///     /// Size variant
///     #[prop(default = ButtonSize::Medium)]
///     size: ButtonSize,
///     /// Whether the button is disabled
///     #[prop(default = false)]
///     disabled: bool,
/// ) -> Markup {
///     html! {
///         button class=(size.class()) disabled[disabled] { (label) }
///     }
/// }
///
/// // Usage:
/// html! {
///     (Button::new("Click me"))
///     (Button::new("Submit").size(ButtonSize::Large))
/// }
/// ```
///
/// The macro generates:
/// - A struct with all props as fields
/// - `new()` constructor with required props as arguments
/// - Builder methods for optional props
/// - `Render` impl for Maud integration (no `.render()` call needed)
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_body = &input_fn.block;
    let fn_docs = extract_doc_comments(&input_fn.attrs);

    // Check return type is Markup
    let _return_type = match &input_fn.sig.output {
        ReturnType::Type(_, ty) => ty,
        ReturnType::Default => {
            return syn::Error::new_spanned(&input_fn.sig, "Component must return Markup")
                .to_compile_error()
                .into();
        }
    };

    // Check if any props have reference types (to determine if we need a lifetime parameter)
    let mut needs_lifetime = false;
    for arg in &input_fn.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            if type_has_references(&pat_type.ty) {
                needs_lifetime = true;
                break;
            }
        }
    }

    // Create the lifetime we'll use if needed
    let synthetic_lifetime: Lifetime = syn::parse_quote!('a);

    // Parse all props from function arguments
    let mut props: Vec<PropInfo> = Vec::new();

    for arg in &input_fn.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            // Extract the parameter name
            let param_name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                pat_ident.ident.clone()
            } else {
                return syn::Error::new_spanned(pat_type, "Expected identifier for parameter")
                    .to_compile_error()
                    .into();
            };

            // Parse #[prop(...)] attribute
            let (is_required_from_attr, default_value) = parse_prop_attr(&pat_type.attrs);

            // Option<T> types are automatically optional with None as default
            let is_option = is_option_type(&pat_type.ty);
            let is_required = is_required_from_attr && !is_option;

            let default_value = if is_option && default_value.is_none() {
                // Option<T> without explicit default gets None
                Some(syn::parse_quote!(None))
            } else if !is_required_from_attr && default_value.is_none() {
                // #[prop(default)] without value uses Default::default()
                Some(syn::parse_quote!(Default::default()))
            } else {
                default_value
            };

            let doc_comments = extract_doc_comments(&pat_type.attrs);
            let doc_string = extract_doc_string(&pat_type.attrs);

            // Get type as string for registry
            let ty_string = pat_type.ty.to_token_stream().to_string();

            // Get default value as string for registry
            let default_string = default_value
                .as_ref()
                .map(|v| v.to_token_stream().to_string());

            // Add lifetime to references if needed
            let ty = if needs_lifetime {
                add_lifetime_to_references(&pat_type.ty, &synthetic_lifetime)
            } else {
                (*pat_type.ty).clone()
            };

            props.push(PropInfo {
                name: param_name,
                ty,
                ty_string,
                is_required,
                default_value,
                default_string,
                doc_comments,
                doc_string,
            });
        }
    }

    // Generate struct fields
    let struct_fields: Vec<_> = props
        .iter()
        .map(|prop| {
            let name = &prop.name;
            let ty = &prop.ty;
            let docs = &prop.doc_comments;
            quote! {
                #(#docs)*
                pub #name: #ty
            }
        })
        .collect();

    // Split props into required and optional
    let required_props: Vec<_> = props.iter().filter(|p| p.is_required).collect();
    let optional_props: Vec<_> = props.iter().filter(|p| !p.is_required).collect();

    // Generate new() parameters (required props only)
    let new_params: Vec<_> = required_props
        .iter()
        .map(|prop| {
            let name = &prop.name;
            let ty = &prop.ty;
            quote! { #name: #ty }
        })
        .collect();

    // Generate struct initialization in new()
    let struct_init_fields: Vec<_> = props
        .iter()
        .map(|prop| {
            let name = &prop.name;
            if prop.is_required {
                quote! { #name }
            } else {
                let default = prop.default_value.as_ref().unwrap();
                quote! { #name: #default }
            }
        })
        .collect();

    // Generate builder methods for optional props
    let builder_methods: Vec<_> = optional_props
        .iter()
        .map(|prop| {
            let name = &prop.name;
            let ty = &prop.ty;
            let docs = &prop.doc_comments;
            quote! {
                #(#docs)*
                pub fn #name(mut self, #name: #ty) -> Self {
                    self.#name = #name;
                    self
                }
            }
        })
        .collect();

    // Generate field access in render body: transform `prop_name` to `self.prop_name`
    // We need to replace bare identifiers with self.ident in the function body
    let prop_names: Vec<_> = props.iter().map(|p| &p.name).collect();
    let transformed_body = transform_body_to_use_self(fn_body, &prop_names);

    // Generate lifetime parameters for struct
    let lifetime_decl = if needs_lifetime {
        quote! { <'a> }
    } else {
        quote! {}
    };

    let lifetime_use = if needs_lifetime {
        quote! { <'a> }
    } else {
        quote! {}
    };

    // For impl blocks, we need the wildcard lifetime if there are lifetimes
    let impl_lifetime = if needs_lifetime {
        quote! { <'_> }
    } else {
        quote! {}
    };

    // Generate prop metadata for registry (strings for static storage)
    let component_name_str = fn_name.to_string();
    let component_doc_str = extract_doc_string(&input_fn.attrs);

    let prop_meta_items: Vec<_> = props
        .iter()
        .map(|prop| {
            let name_str = prop.name.to_string();
            let ty_str = &prop.ty_string;
            let required = prop.is_required;
            let default_str = match &prop.default_string {
                Some(s) => quote! { Some(#s) },
                None => quote! { None },
            };
            let doc_str = match &prop.doc_string {
                Some(s) => quote! { Some(#s) },
                None => quote! { None },
            };
            quote! {
                rejoice::studio::PropMeta {
                    name: #name_str,
                    ty: #ty_str,
                    required: #required,
                    default: #default_str,
                    doc: #doc_str,
                }
            }
        })
        .collect();

    let component_doc_token = match &component_doc_str {
        Some(s) => quote! { Some(#s) },
        None => quote! { None },
    };

    // Generate a unique identifier for the static props array (uppercase for lint)
    let fn_name_upper = fn_name.to_string().to_uppercase();
    let props_static_name = syn::Ident::new(
        &format!("__REJOICE_PROPS_{}", fn_name_upper),
        fn_name.span(),
    );

    let registration_static_name = syn::Ident::new(
        &format!("__REJOICE_REGISTERED_{}", fn_name_upper),
        fn_name.span(),
    );

    // Generate the output
    let output = quote! {
        #(#fn_docs)*
        #fn_vis struct #fn_name #lifetime_decl {
            #(#struct_fields),*
        }

        // Static prop metadata array (only used in debug builds)
        #[cfg(debug_assertions)]
        static #props_static_name: &[rejoice::studio::PropMeta] = &[
            #(#prop_meta_items),*
        ];

        // Registration flag to ensure we only register once
        #[cfg(debug_assertions)]
        static #registration_static_name: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);

        impl #lifetime_decl #fn_name #lifetime_use {
            /// Create a new instance with required props.
            pub fn new(#(#new_params),*) -> Self {
                Self {
                    #(#struct_init_fields),*
                }
            }

            #(#builder_methods)*

            /// Register this component in the Studio registry (debug builds only).
            #[cfg(debug_assertions)]
            fn __register_component() {
                use std::sync::atomic::Ordering;
                if !#registration_static_name.swap(true, Ordering::SeqCst) {
                    rejoice::studio::register_component(rejoice::studio::ComponentMeta {
                        name: #component_name_str,
                        file: file!(),
                        line: line!(),
                        column: column!(),
                        doc: #component_doc_token,
                        props: #props_static_name,
                    });
                }
            }

            #[cfg(not(debug_assertions))]
            fn __register_component() {}
        }

        impl maud::Render for #fn_name #impl_lifetime {
            fn render(&self) -> maud::Markup {
                // Register component on first render
                #fn_name::__register_component();

                let __inner_content: maud::Markup = #transformed_body;

                // In debug mode, wrap with data attributes for Studio
                #[cfg(debug_assertions)]
                {
                    maud::html! {
                        div
                            data-component=#component_name_str
                            data-source=(format!("{}:{}:{}", file!(), line!(), column!()))
                        {
                            (__inner_content)
                        }
                    }
                }

                #[cfg(not(debug_assertions))]
                {
                    __inner_content
                }
            }
        }
    };

    output.into()
}

/// Transform the function body to prefix prop names with `self.`
fn transform_body_to_use_self(
    body: &syn::Block,
    prop_names: &[&syn::Ident],
) -> proc_macro2::TokenStream {
    use quote::ToTokens;

    let body_tokens = body.to_token_stream();
    transform_tokens(body_tokens, prop_names)
}

fn transform_tokens(
    tokens: proc_macro2::TokenStream,
    prop_names: &[&syn::Ident],
) -> proc_macro2::TokenStream {
    use proc_macro2::{TokenStream, TokenTree};

    let tokens_vec: Vec<_> = tokens.into_iter().collect();
    let mut result = TokenStream::new();
    let mut i = 0;

    while i < tokens_vec.len() {
        let token = &tokens_vec[i];

        match token {
            TokenTree::Group(group) => {
                // Recursively transform inside groups (parens, braces, brackets)
                let inner = transform_tokens(group.stream(), prop_names);
                let mut new_group = proc_macro2::Group::new(group.delimiter(), inner);
                new_group.set_span(group.span());
                result.extend(std::iter::once(TokenTree::Group(new_group)));
            }
            TokenTree::Punct(punct) if punct.as_char() == '.' => {
                // This is a dot - emit it and skip transformation of the following identifier
                // because it's a method call or field access, not a variable reference
                result.extend(std::iter::once(token.clone()));
                i += 1;
                // Emit the next token as-is (the method/field name after the dot)
                if i < tokens_vec.len() {
                    // But still need to recurse into groups
                    match &tokens_vec[i] {
                        TokenTree::Group(group) => {
                            let inner = transform_tokens(group.stream(), prop_names);
                            let mut new_group = proc_macro2::Group::new(group.delimiter(), inner);
                            new_group.set_span(group.span());
                            result.extend(std::iter::once(TokenTree::Group(new_group)));
                        }
                        other => {
                            result.extend(std::iter::once(other.clone()));
                        }
                    }
                }
            }
            TokenTree::Ident(ident) => {
                // Check if this identifier is a prop name
                let is_prop = prop_names.iter().any(|p| *p == ident);

                if is_prop {
                    // Replace `prop` with `self.prop`
                    let self_token: TokenStream = quote! { self. };
                    result.extend(self_token);
                }
                result.extend(std::iter::once(token.clone()));
            }
            _ => {
                result.extend(std::iter::once(token.clone()));
            }
        }

        i += 1;
    }

    result
}

/// Derive macro for enum props to enable storybook dropdown generation.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(PropEnum)]
/// pub enum ButtonSize {
///     Small,
///     Medium,
///     Large,
/// }
/// ```
///
/// Generates a `PropEnum` impl that provides:
/// - `variants()` - list of variant names
/// - `from_variant_name()` - construct from string
#[proc_macro_derive(PropEnum)]
pub fn derive_prop_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract enum variants
    let variants = match &input.data {
        syn::Data::Enum(data_enum) => &data_enum.variants,
        _ => {
            return syn::Error::new_spanned(&input, "PropEnum can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    // Check that all variants are unit variants (no fields)
    for variant in variants {
        if !variant.fields.is_empty() {
            return syn::Error::new_spanned(
                variant,
                "PropEnum only supports unit variants (no fields)",
            )
            .to_compile_error()
            .into();
        }
    }

    let variant_names: Vec<_> = variants.iter().map(|v| &v.ident).collect();
    let variant_strings: Vec<_> = variant_names.iter().map(|v| v.to_string()).collect();

    let output = quote! {
        impl #name {
            /// Returns a list of all variant names as strings.
            pub fn prop_enum_variants() -> &'static [&'static str] {
                &[#(#variant_strings),*]
            }

            /// Construct a variant from its string name.
            pub fn prop_enum_from_name(name: &str) -> Option<Self> {
                match name {
                    #(#variant_strings => Some(Self::#variant_names),)*
                    _ => None,
                }
            }

            /// Get the name of this variant as a string.
            pub fn prop_enum_name(&self) -> &'static str {
                match self {
                    #(Self::#variant_names => #variant_strings,)*
                }
            }
        }
    };

    output.into()
}
