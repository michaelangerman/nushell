pub mod camel_case;
pub mod kebab_case;
pub mod pascal_case;
pub mod screaming_snake_case;
pub mod snake_case;

use crate::prelude::*;
use nu_errors::ShellError;
use nu_protocol::ShellTypeName;
use nu_protocol::{ColumnPath, Primitive, UntaggedValue, Value};
use nu_source::Tag;
use nu_value_ext::ValueExt;

pub use camel_case::SubCommand as CamelCase;
pub use pascal_case::SubCommand as PascalCase;
pub use screaming_snake_case::SubCommand as ScreamingSnakeCase;
pub use snake_case::SubCommand as SnakeCase;

use heck::ToKebabCase;
use heck::ToLowerCamelCase;
use heck::ToShoutySnakeCase;
use heck::ToSnakeCase;
use heck::ToUpperCamelCase;
macro_rules! create_heck_function {
    ($func_name:ident) => {
        pub fn $func_name(a_slice: &str) -> String {
            a_slice.$func_name()
        }
    };
}
create_heck_function!(to_upper_camel_case);
create_heck_function!(to_lower_camel_case);
create_heck_function!(to_kebab_case);
create_heck_function!(to_shouty_snake_case);
create_heck_function!(to_snake_case);

struct Arguments {
    column_paths: Vec<ColumnPath>,
}

pub fn operate<F>(args: CommandArgs, case_operation: &'static F) -> Result<OutputStream, ShellError>
where
    F: Fn(&str) -> String + Send + Sync + 'static,
{
    let (options, input) = (
        Arguments {
            column_paths: args.rest(0)?,
        },
        args.input,
    );

    Ok(input
        .map(move |v| {
            if options.column_paths.is_empty() {
                action(&v, v.tag(), &case_operation)
            } else {
                let mut ret = v;

                for path in &options.column_paths {
                    ret = ret.swap_data_by_column_path(
                        path,
                        Box::new(move |old| action(old, old.tag(), &case_operation)),
                    )?;
                }

                Ok(ret)
            }
        })
        .into_input_stream())
}

pub fn action<F>(
    input: &Value,
    tag: impl Into<Tag>,
    case_operation: &F,
) -> Result<Value, ShellError>
where
    F: Fn(&str) -> String + Send + Sync + 'static,
{
    match &input.value {
        UntaggedValue::Primitive(Primitive::String(s)) => {
            Ok(UntaggedValue::string(case_operation(s)).into_value(tag))
        }
        other => {
            let got = format!("got {}", other.type_name());
            Err(ShellError::labeled_error(
                "value is not string",
                got,
                tag.into().span,
            ))
        }
    }
}
