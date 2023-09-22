import gleam/bool
import gleam/list
import gleam/regex
import gleam/string
import phony/metadata

pub type ValidationResult {
  ValidationResult(country: Country, kind: PhoneNumberKind)
}

pub type Country {
  Country(name: String, alpha2: String, code: String)
}

pub type PhoneNumberKind {
  Mobile
  Landline
}

pub fn get_countries() {
  metadata.by_alpha2
  |> list.map(fn(metadata) {
    let alpha2 = metadata.0
    let #(name, code, _, _, _) = metadata.1
    Country(name, alpha2, code)
  })
}

pub fn validate(phone_number: String) -> Result(ValidationResult, Nil) {
  use <- bool.guard(pre_validate(phone_number), Error(Nil))

  case string.starts_with(phone_number, "+") {
    True -> {
      let phone_number = string.drop_left(phone_number, 1)

      case string.split_once(phone_number, " ") {
        Ok(#(code, phone_number)) ->
          case string.length(code) <= 3 {
            True -> validate_by_code(phone_number, code)
            False -> try_various_code_lengths(code <> phone_number)
          }

        Error(Nil) -> try_various_code_lengths(phone_number)
      }
    }

    False -> walk_metadata(phone_number)
  }
}

pub fn validate_by_country(
  phone_number: String,
  alpha2: String,
) -> Result(ValidationResult, Nil) {
  use <- bool.guard(pre_validate(phone_number), Error(Nil))

  case
    metadata.by_alpha2
    |> list.key_find(alpha2)
  {
    Ok(metadata) -> {
      let #(country, code, mobile_pattern, landline_pattern, possible_lengths) =
        metadata

      use <- bool.guard(
        !list.contains(possible_lengths, string.length(phone_number)),
        Error(Nil),
      )

      let assert Ok(pattern) = regex.from_string(mobile_pattern)
      case regex.check(pattern, phone_number) {
        True ->
          ValidationResult(Country(country, metadata.0, code), Mobile)
          |> Ok

        False -> {
          let assert Ok(pattern) = regex.from_string(landline_pattern)
          case regex.check(pattern, phone_number) {
            True ->
              ValidationResult(Country(country, metadata.0, code), Landline)
              |> Ok

            False -> Error(Nil)
          }
        }
      }
    }

    Error(Nil) -> Error(Nil)
  }
}

pub fn validate_by_code(
  phone_number: String,
  code: String,
) -> Result(ValidationResult, Nil) {
  use <- bool.guard(pre_validate(phone_number), Error(Nil))

  case
    metadata.by_code
    |> list.filter(fn(metadata) { metadata.0 == code })
  {
    [] -> Error(Nil)
    filtered -> {
      list.find_map(
        filtered,
        fn(metadata) {
          let #(
            country,
            alpha2,
            mobile_pattern,
            landline_pattern,
            possible_lengths,
          ) = metadata.1

          use <- bool.guard(
            !list.contains(possible_lengths, string.length(phone_number)),
            Error(Nil),
          )

          let assert Ok(pattern) = regex.from_string(mobile_pattern)
          case regex.check(pattern, phone_number) {
            True ->
              ValidationResult(Country(country, alpha2, code), Mobile)
              |> Ok

            False -> {
              let assert Ok(pattern) = regex.from_string(landline_pattern)
              case regex.check(pattern, phone_number) {
                True ->
                  ValidationResult(Country(country, alpha2, code), Landline)
                  |> Ok

                False -> Error(Nil)
              }
            }
          }
        },
      )
    }
  }
}

fn try_various_code_lengths(phone_number: String) {
  let code = string.slice(phone_number, 0, 1)
  let phone_number = string.drop_left(phone_number, 1)

  case validate_by_code(phone_number, code) {
    Ok(result) -> Ok(result)
    Error(Nil) -> {
      let phone_number = code <> phone_number

      let code = string.slice(phone_number, 0, 2)
      let phone_number = string.drop_left(phone_number, 2)

      case validate_by_code(phone_number, code) {
        Ok(result) -> Ok(result)
        Error(Nil) -> {
          let phone_number = code <> phone_number

          let code = string.slice(phone_number, 0, 3)
          let phone_number = string.drop_left(phone_number, 3)

          case validate_by_code(phone_number, code) {
            Ok(result) -> Ok(result)
            Error(Nil) -> {
              let phone_number = code <> phone_number
              walk_metadata(phone_number)
            }
          }
        }
      }
    }
  }
}

fn walk_metadata(phone_number: String) {
  metadata.by_alpha2
  |> list.find_map(fn(metadata) {
    let #(country, code, mobile_pattern, landline_pattern, possible_lengths) =
      metadata.1

    use <- bool.guard(
      !list.contains(possible_lengths, string.length(phone_number)),
      Error(Nil),
    )

    let assert Ok(pattern) = regex.from_string(mobile_pattern)
    case regex.check(pattern, phone_number) {
      True ->
        ValidationResult(Country(country, metadata.0, code), Mobile)
        |> Ok

      False -> {
        let assert Ok(pattern) = regex.from_string(landline_pattern)
        case regex.check(pattern, phone_number) {
          True ->
            ValidationResult(Country(country, metadata.0, code), Landline)
            |> Ok

          False -> Error(Nil)
        }
      }
    }
  })
}

fn pre_validate(phone_number) -> Bool {
  let assert Ok(pattern) = regex.from_string("[A-Za-z]")
  regex.check(pattern, phone_number)
}
