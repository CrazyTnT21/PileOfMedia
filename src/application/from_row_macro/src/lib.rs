use proc_macro::{TokenStream, TokenTree};

use quote::quote;
use syn::{Attribute, Data, DeriveInput, Expr, Fields, Ident, Lit, Meta, parse_macro_input, Type};
use syn::__private::Span;

#[proc_macro_derive(FromRow, attributes(rename))]
pub fn from_row(item: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(item as DeriveInput);
  from_row_macro_impl(&ast)
}

struct DbColumnIdent {
  field: Ident,
  rename: Option<String>,
  is_optional: bool,
}

fn is_optional(ty: &Type) -> bool {
  let Type::Path(path) = ty else {
    return false;
  };
  path.path.segments[0].ident.to_string() == "Option"
}

fn renamed_field(attributes: &Vec<Attribute>) -> Option<String> {
  for attribute in attributes {
    let Meta::NameValue(name_value) = &attribute.meta else {
      panic!("Invalid symbol. Rename only allows \"=\"")
    };
    let Expr::Lit(lit) = &name_value.value else {
      panic!("Invalid Expression. Rename only allows strings using \"=\"")
    };
    let Lit::Str(value) = &lit.lit else {
      panic!("Invalid value. Rename only allows strings")
    };
    return Some(value.value());
  }

  None
}

fn from_row_impl(name: &Ident, db_mapping: &Vec<DbColumnIdent>) -> proc_macro2::TokenStream {
  let fields = db_mapping.iter().clone().enumerate().map(|(index, mapping)|
    {
      let field = &mapping.field;
      quote! { #field: row.get(#index + from) }
    });

  let optionals = db_mapping.iter().enumerate().map(|(index, mapping)|
    {
      let field = &mapping.field;
      match mapping.is_optional {
        true => quote! { #field: row.get(#index + from)},
        false => quote! { #field: row.try_get(#index + from).ok()? }
      }
    });

  quote!(
    impl from_row::FromRow for #name{
       type DbType = #name;
       fn from_row(row: &Row, from: usize) -> Self::DbType {
        #name {
          #(#fields),*
        }
      }
      fn from_row_optional(row: &Row, from: usize) -> Option<Self::DbType> {
        Some(#name {
          #(#optionals),*
        })
      }
    }
  )
}

fn from_row_macro_impl(ast: &DeriveInput) -> TokenStream {
  let Data::Struct(data) = &ast.data else {
    panic!("FromRow is only supported for structs!")
  };
  let Fields::Named(named_field) = &data.fields else {
    panic!("FromRow only supports named fields.")
  };

  let db_mapping: Vec<DbColumnIdent> = named_field.clone().named.iter().map(|x| DbColumnIdent {
    rename: renamed_field(&x.attrs),
    field: x.clone().ident.unwrap(),
    is_optional: is_optional(&x.ty),
  }).collect::<Vec<DbColumnIdent>>();

  let columns = db_mapping.iter().map(|x| match &x.rename {
    None => x.field.clone().to_string(),
    Some(value) => value.clone()
  }).collect::<Vec<String>>();

  let from_row_impl = from_row_impl(&ast.ident, &db_mapping);
  let columns_impl = row_columns_impl(&ast.ident, &columns);
  let gen = quote! {
    #from_row_impl
    #columns_impl
  };

  gen.into()
}

fn row_columns_impl(name: &Ident, columns: &Vec<String>) -> proc_macro2::TokenStream {
  quote!(
    impl from_row::RowColumns for #name {
      fn columns() -> Vec<&'static str>{
        vec![#(#columns),*]
      }
    }
  )
}

#[proc_macro]
pub fn query_row(item: TokenStream) -> TokenStream {
  let mapped = item.into_iter().filter_map(|x| match x {
    TokenTree::Ident(ident) => Some(ident),
    _ => None
  }).collect::<Vec<proc_macro::Ident>>();

  let mut items: Vec<(&proc_macro::Ident, bool)> = vec![];

  let mut iterator = mapped.iter();

  while let Some(value) = iterator.next() {
    if value.to_string() != "Option" {
      items.push((value, false));
      continue;
    }
    let Some(next_value) = iterator.next() else {
      break;
    };

    items.push((next_value, true));
  }
  let row_ident = Ident::new(&items[0].0.to_string(), Span::call_site());
  let types = items[1..].iter().map(|(x, optional)|
    {
      //convert from proc_macro::Ident to proc_macro2::Ident
      let x = Ident::new(&x.to_string(), Span::call_site());
      match *optional {
        true => quote!(<#x as from_row::FromRow>::from_row_optional(&#row_ident, start(<#x as from_row::RowColumns>::columns().len()))),
        false => quote!(<#x as from_row::FromRow>::from_row(&#row_ident, start(<#x as from_row::RowColumns>::columns().len())))
      }
    });

  quote!({
    let mut current_start = 0;
    let mut start = |x| {
      let current = current_start;
      current_start += x;
      current
    };
    (#(#types),*,)
  }).into()
}
