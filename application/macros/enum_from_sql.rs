#[macro_export]
macro_rules! enum_from_sql {
  ($x: tt,$db_name: literal) => {
    impl<'a> FromSql<'a> for $x {
      fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let value = $x::from_str(std::str::from_utf8(raw)?)?;
        Ok(value.into())
      }

      fn accepts(ty: &Type) -> bool {
        if ty.name() != $db_name {
          return false;
        }
        let Kind::Enum(value) = ty.kind() else {
          return false;
        };
        for x in value {
          match $x::from_str(x) {
            Ok(_) => {}
            Err(_) => {
              return false;
            }
          }
        }
        true
      }
    }
  };
}
