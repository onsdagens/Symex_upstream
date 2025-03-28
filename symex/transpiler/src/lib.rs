//! Defines a transpiler that allows inline pseudo code
//! to be translated in to [`general_assembly`]
extern crate proc_macro;

use language::{ast::IR, TypeCheck, TypeCheckMeta};
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
/// Extends or creates a vector of [`general_assembly`] operations.
///
/// Usage:
/// ```
/// use general_assembly::{operation::Operation,operand::Operand,condition::Condition};
/// use transpiler::pseudo;
///
/// let a = Operand::Register("a".to_owned());
/// let b = Operand::Register("b".to_owned());
/// let c = Operand::Local("c".to_owned());
/// let cond = false;
/// let ret = pseudo!([
///     a:u32;b:u32;
///     c:u32 = a+b;
///     let d = a ^ b;
///     
///     if(cond) {
///         d = a | b;
///     }
///     
///     c = d;
///     Jump(c);
/// ]);
/// ```
pub fn pseudo(item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as IR);
    //println!("Output \n{:?}\n\n\n", input);
    if let Err(e) = input.type_check(&mut TypeCheckMeta::new()) {
        //let inner = format!("{:?}", e);
        return e.compile_error().into();
    };

    // TODO: Filter out noops.
    //input.filter();
    let input: proc_macro2::TokenStream = match input.into() {
        Ok(val) => val,
        Err(e) => return e.compile_error().into(),
    };

    input.into()
}
