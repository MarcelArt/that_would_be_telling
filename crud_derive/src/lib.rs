use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput, Ident};
use quote::quote;

fn get_entity_and_dto(input: &DeriveInput) -> (Ident, Ident) {
    let mut entity_type = None;
    let mut dto_type = None;

    for attr in &input.attrs {
        if attr.path().is_ident("entity") {
            if let Ok(lit) = attr.parse_args::<syn::LitStr>() {
                entity_type = Some(syn::Ident::new(&lit.value(), lit.span()));
            }
        }
        if attr.path().is_ident("dto") {
            if let Ok(lit) = attr.parse_args::<syn::LitStr>() {
                dto_type = Some(syn::Ident::new(&lit.value(), lit.span()));
            }
        }
    }

    (
        entity_type.expect("msg: entity type not specified"), 
        dto_type.expect("msg: dto type not specified")
    )
}

#[proc_macro_derive(ICreate, attributes(entity, dto))]
pub fn derive_icreate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Get the repository struct name (e.g., "PermissionRepo")
    let repo_name = &input.ident;
    
    let (entity_type, dto_type) = get_entity_and_dto(&input);

    let expanded = quote! {
        impl ICreate<#entity_type, #dto_type> for #repo_name {
            async fn create(&self, input: #dto_type) -> Result<Option<#entity_type>, crate::error::Error> {
                let result = self.db.create(self.table_name.clone()).content(input).await?;
                Ok(result)
            }
        }
    };
    
    TokenStream::from(expanded)
}

#[proc_macro_derive(IRead, attributes(entity))]
pub fn derive_iread(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Get the repository struct name (e.g., "PermissionRepo")
    let repo_name = &input.ident;
    
    let (entity_type, _) = get_entity_and_dto(&input);

    let expanded = quote! {
        impl IRead<#entity_type> for #repo_name {
            async fn read(&self) -> Result<Vec<#entity_type>, crate::error::Error> {
                let result = self.db.select(self.table_name.clone()).await?;
                Ok(result)
            }
        }
    };
    
    TokenStream::from(expanded)
}

#[proc_macro_derive(IUpdate, attributes(entity, dto))]
pub fn derive_iupdate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Get the repository struct name (e.g., "PermissionRepo")
    let repo_name = &input.ident;
    
    let (entity_type, dto_type) = get_entity_and_dto(&input);

    let expanded = quote! {
        impl IUpdate<#entity_type, #dto_type> for #repo_name {
            async fn update(&self, id: String, input: #dto_type) -> Result<Option<#entity_type>, crate::error::Error> {
                let result = self.db.update((self.table_name.clone(), &id)).content(input).await?;
                Ok(result)
            }
        }
    };
    
    TokenStream::from(expanded)
}

#[proc_macro_derive(IDelete, attributes(entity))]
pub fn derive_idelete(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Get the repository struct name (e.g., "PermissionRepo")
    let repo_name = &input.ident;
    
    let (entity_type, _) = get_entity_and_dto(&input);

    let expanded = quote! {
        impl IDelete<#entity_type> for #repo_name {
            async fn delete(&self, id: String) -> Result<Option<#entity_type>, crate::error::Error> {
                let result = self.db.delete((self.table_name.clone(), &id)).await?;
                Ok(result)
            }
        }
    };
    
    TokenStream::from(expanded)
}

#[proc_macro_derive(IGetById, attributes(entity))]
pub fn derive_iget_by_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Get the repository struct name (e.g., "PermissionRepo")
    let repo_name = &input.ident;
    
    let (entity_type, _) = get_entity_and_dto(&input);

    let expanded = quote! {
        impl IGetById<#entity_type> for #repo_name {
            async fn get_by_id(&self, id: String) -> Result<Option<#entity_type>, crate::error::Error> {
                let result = self.db.select((self.table_name.clone(), &id)).await?;
                Ok(result)
            }
        }
    };
    
    TokenStream::from(expanded)
}