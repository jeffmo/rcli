pub struct ParamMetadata {
  pub aliases: Vec<String>,
  pub allow_multiple: bool,
  pub help: String,
  pub name: String,
  pub required: bool,
  //parser: Fn     // TODO: This might get tricky with generics and such
}

pub fn nameify_param(param_key: &str) -> String {
  String::from(param_key).replace("_", "-")
}

pub trait CLIArgs {
  fn __rcli_get_metadata() -> std::collections::HashMap<String, ParamMetadata>;  

  // TODO: Next big step!
  // fn __rcli_parse() -> Self;
}

/*
pub fn parse_args<T: CLIArgs>(argv: Vec<String>) -> T {
  let metadata = T::__rcli_get_metadata();
}
*/

#[macro_export]
macro_rules! __rcli_expand_struct {
  ($struct_name:ident, {
    $($param_key:ident : $param_type:ty => $param_opts:tt),*
    $(,)*
  }) => {
    struct $struct_name {
      $(
        $param_key: $param_type,
      )*
    }
  }
}

#[macro_export]
macro_rules! __rcli_expand_struct_impl {
  ($struct_name:ident, {
    $($param_key:ident : $param_type:ty => {
      $($opt_ident:tt = $opt_value:expr),*
      $(,)*
    }),*
    $(,)*
  }) => {
    impl rcli::CLIArgs for $struct_name {
      fn __rcli_get_metadata() -> std::collections::HashMap<String, rcli::ParamMetadata> {
        let mut map = std::collections::HashMap::new();
        $(
          map.insert(String::from(stringify!($param_key)), {
            let mut metadata = rcli::ParamMetadata {
              aliases: vec![],
              allow_multiple: false,
              help: String::from(""),
              name: rcli::nameify_param(stringify!($param_key)),
              required: false,
            };
            $(
              metadata.$opt_ident = $opt_value;
            )*
            metadata
          });
        )*
        map
      }
    }
  }
}

#[macro_export]
macro_rules! rcli {
  (struct $struct_name:ident $params:tt) => {
    __rcli_expand_struct!($struct_name, $params);
    //trace_macros!(true);
    __rcli_expand_struct_impl!($struct_name, $params);
    //trace_macros!(false);
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
