import gleeunit
import gleeunit/should
import phony

pub fn main() {
  gleeunit.main()
}

pub fn us_mobile_test() {
  phony.validate("+1 4305573966")
  |> should.be_ok
  |> should.equal(phony.ValidationResult(
    phony.Country("United States of America", "US", "1"),
    phony.Mobile,
  ))
}

pub fn de_landline_test() {
  phony.validate("+49 1522 343333")
  |> should.be_ok
  |> should.equal(phony.ValidationResult(
    phony.Country("Germany", "DE", "49"),
    phony.Landline,
  ))
}

pub fn fr_landline_test() {
  phony.validate("+3314 2685300")
  |> should.be_ok
  |> should.equal(phony.ValidationResult(
    phony.Country("France", "FR", "33"),
    phony.Landline,
  ))
}

pub fn fi_mobile_test() {
  phony.validate("+358425323456")
  |> should.be_ok
  |> should.equal(phony.ValidationResult(
    phony.Country("Finland", "FI", "358"),
    phony.Mobile,
  ))
}

pub fn ir_mobile_test() {
  phony.validate_by_country("09101752439", "IR")
  |> should.be_ok
  |> should.equal(phony.ValidationResult(
    phony.Country("Iran (Islamic Republic of)", "IR", "98"),
    phony.Mobile,
  ))
}

pub fn ir_landline_test() {
  phony.validate_by_country("02833772434", "IR")
  |> should.be_ok
  |> should.equal(phony.ValidationResult(
    phony.Country("Iran (Islamic Republic of)", "IR", "98"),
    phony.Landline,
  ))
}

pub fn bd_mobile_test() {
  phony.validate_by_code("1812345678", "880")
  |> should.be_ok
  |> should.equal(phony.ValidationResult(
    phony.Country("Bangladesh", "BD", "880"),
    phony.Mobile,
  ))
}

pub fn bd_landline_test() {
  phony.validate_by_code("27111234", "880")
  |> should.be_ok
  |> should.equal(phony.ValidationResult(
    phony.Country("Bangladesh", "BD", "880"),
    phony.Landline,
  ))
}

pub fn invalid_us_test() {
  phony.validate("+127111234")
  |> should.be_error
}

pub fn invalid_de_test() {
  phony.validate("+49 321000")
  |> should.be_error
}

pub fn invalid_fr_test() {
  phony.validate("+33321000")
  |> should.be_error
}

pub fn invalid_fi_test() {
  phony.validate("+35 410 1234")
  |> should.be_error
}

pub fn invalid_ir_test() {
  phony.validate("+984101234")
  |> should.be_error
}

pub fn invalid_zm_test() {
  phony.validate("+260101234234")
  |> should.be_error
}

pub fn invalid_jp_test() {
  phony.validate_by_country("32 10 00", "JP")
  |> should.be_error
}

pub fn invalid_bd_test() {
  phony.validate_by_code("410 12 34", "880")
  |> should.be_error
}
