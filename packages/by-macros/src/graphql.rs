use convert_case::Casing;
use indexmap::IndexMap;
use quote::quote;

pub fn generate_graphql(m: &crate::api_model_struct::ApiModel) -> proc_macro2::TokenStream {
    if !m.graphql || m.database.is_none() {
        return quote! {};
    }

    let struct_name = m.struct_name();

    let lowercase_struct_name = struct_name.to_string().to_case(convert_case::Case::Snake);
    let plural_struct_name = format!("{}s", lowercase_struct_name);
    let endpoint_name = format!("{}Endpoints", struct_name.to_string());
    let lowercase_struct_name =
        syn::Ident::new(&lowercase_struct_name, proc_macro2::Span::call_site());
    let plural_struct_name = syn::Ident::new(&plural_struct_name, proc_macro2::Span::call_site());
    let endpoint_name = syn::Ident::new(&endpoint_name, proc_macro2::Span::call_site());

    let mut aggregate_args: IndexMap<String, proc_macro2::TokenStream> = IndexMap::new();
    let mut arg_names = vec![];
    let mut field_names = vec![];
    let mut field_types = vec![];
    let mut query_functions = vec![];

    for (_, field) in m.fields.iter() {
        for (name, q) in field.aggregate_arg() {
            aggregate_args.insert(name.clone(), q);

            let arg_name = syn::Ident::new(&name, proc_macro2::Span::call_site());
            arg_names.push(arg_name);
        }

        if field.skip || field.relation.is_some() {
            continue;
        }

        let ty = if field.rust_type.starts_with("Option <") {
            field
                .rust_type
                .trim_start_matches("Option <")
                .trim_end_matches('>')
                .to_string()
        } else {
            field.rust_type.to_string()
        };

        if ty.eq("bool") {
            // FIXME: temporarily skip bool fields
            continue;
        }

        field_names.push(syn::Ident::new(&field.name, proc_macro2::Span::call_site()));

        let f = syn::Ident::new(
            &format!("{}_equals", field.name),
            proc_macro2::Span::call_site(),
        );
        let var = syn::Ident::new(&format!("{}", field.name), proc_macro2::Span::call_site());

        query_functions.push(quote! {
            if let Some(value) = #var {
                q = q.#f(value);
            }
        });

        field_types.push(syn::Ident::new(&ty, proc_macro2::Span::call_site()));
    }

    let aggregate_args: Vec<proc_macro2::TokenStream> = aggregate_args.into_values().collect();

    let output = quote! {
        pub struct #endpoint_name;

        #[async_graphql::Object]
        impl #endpoint_name {
            // async fn #lowercase_struct_name<'a>(&self, ctx: &async_graphql::Context<'a>, #(#aggregate_args),*) -> async_graphql::Result<#struct_name> {
            //     let pool = ctx
            //         .data::<sqlx::Pool<sqlx::Postgres>>()
            //         .expect("Database pool not found");

            //     let q = #struct_name::query_builder(#(#arg_names),*);
            //     let doc = q.query()
            //         .map(#struct_name::from)
            //         .fetch_one(pool)
            //         .await?;

            //     Ok(doc)

            // }

            // async fn #plural_struct_name<'a>(&self, ctx: &async_graphql::Context<'a>, size: i32, page: i32, #(#aggregate_args),* ) -> async_graphql::Result<Vec<#struct_name>> {
            //     let pool = ctx
            //         .data::<sqlx::Pool<sqlx::Postgres>>()
            //         .expect("Database pool not found");

            //     let q = #struct_name::query_builder(#(#arg_names),*);
            //     let docs = q
            //         .limit(size)
            //         .page(page)
            //         .query()
            //         .map(#struct_name::from)
            //         .fetch_all(pool)
            //         .await?;

            //     Ok(docs)
            // }

            async fn #lowercase_struct_name<'a>(&self, ctx: &async_graphql::Context<'a>, #(#aggregate_args),* #(#field_names: Option< #field_types>),*) -> async_graphql::Result<#struct_name> {
                let pool = ctx
                    .data::<sqlx::Pool<sqlx::Postgres>>()?;

                let mut q = #struct_name::query_builder(#(#arg_names),*);
                #(#query_functions)*

                let doc = q.query()
                    .map(#struct_name::from)
                    .fetch_one(pool)
                    .await?;

                Ok(doc)

            }

            async fn #plural_struct_name<'a>(&self, ctx: &async_graphql::Context<'a>, size: i32, page: i32, #(#aggregate_args),* #(#field_names: Option< #field_types>),*) -> async_graphql::Result<Vec<#struct_name>> {
                let pool = ctx
                    .data::<sqlx::Pool<sqlx::Postgres>>()?;

                let mut q = #struct_name::query_builder(#(#arg_names),*);

                #(#query_functions)*

                let docs = q
                    .limit(size)
                    .page(page)
                    .query()
                    .map(#struct_name::from)
                    .fetch_all(pool)
                    .await?;

                Ok(docs)
            }

        }

    };

    println!(
        "Generated GraphQL for {}: {}",
        struct_name,
        output.to_string()
    );

    output.into()
}
