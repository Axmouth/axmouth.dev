use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(PaginatedQuery, attributes(page, page_size))]
pub fn derive_paginated_query_attr(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_paginated_query_macro(&ast)
}

fn impl_paginated_query_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl PaginatedQueryExt for #name {
            fn paginated_query(&self) {
                Pagination {
                    self.page,
                    self.page_size,
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(ExtractedQuery, attributes())]
pub fn derive_extracted_query_attr(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_extracted_query_macro(&ast)
}

fn impl_extracted_query_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[::axum_derive::axum::async_trait]
        impl<B> ::axum_derive::axum::extract::FromRequest<B> for #name
        where
            B: ::axum_derive::http_body::Body + Send,
            B::Data: Send,
            B::Error: Into<::axum_derive::axum::BoxError>,
        {

            type Rejection = ::axum_derive::axum::extract::rejection::QueryRejection;

            async fn from_request(req: &mut ::axum_derive::axum::extract::RequestParts<B>) -> Result<Self, Self::Rejection> {
                let ::axum_derive::axum::extract::Query(value) = ::axum_derive::axum::extract::Query::<#name>::from_request(req).await?;
                Ok(value)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(ValidatedExtractedQuery, attributes())]
pub fn derive_validated_extracted_query_attr(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_validated_extracted_query_macro(&ast)
}

fn impl_validated_extracted_query_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[::axum_derive::axum::async_trait]
        impl<B> ::axum_derive::axum::extract::FromRequest<B> for #name
        where
            B: ::axum_derive::http_body::Body + Send,
            B::Data: Send,
            B::Error: Into<::axum_derive::axum::BoxError>,
        {

            type Rejection = ::axum_derive::ValidatedFormRejection;

            async fn from_request(req: &mut ::axum_derive::axum::extract::RequestParts<B>) -> Result<Self, Self::Rejection> {
                let ::axum_derive::axum::extract::Query(value) = ::axum_derive::axum::extract::Query::<#name>::from_request(req).await?;
                ::axum_derive::validator::Validate::validate(&value)?;
                Ok(value)
            }
        }
    };
    gen.into()
}
