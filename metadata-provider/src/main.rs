use std::io::{BufReader, Cursor, Write};

mod error;
mod loader;

const METADATA_URL: &str =
    "https://github.com/google/libphonenumber/raw/master/resources/PhoneNumberMetadata.xml";

const GLEAM_FILE_PATH: &str = "src/phony/metadata.gleam";

const COUNTRIES: [(&str, &str); 249] = [
    ("AD", "Andorra"),
    ("AE", "United Arab Emirates"),
    ("AF", "Afghanistan"),
    ("AG", "Antigua and Barbuda"),
    ("AI", "Anguilla"),
    ("AL", "Albania"),
    ("AM", "Armenia"),
    ("AO", "Angola"),
    ("AQ", "Antarctica"),
    ("AR", "Argentina"),
    ("AS", "American Samoa"),
    ("AT", "Austria"),
    ("AU", "Australia"),
    ("AW", "Aruba"),
    ("AX", "Åland Islands"),
    ("AZ", "Azerbaijan"),
    ("BA", "Bosnia and Herzegovina"),
    ("BB", "Barbados"),
    ("BD", "Bangladesh"),
    ("BE", "Belgium"),
    ("BF", "Burkina Faso"),
    ("BG", "Bulgaria"),
    ("BH", "Bahrain"),
    ("BI", "Burundi"),
    ("BJ", "Benin"),
    ("BL", "Saint Barthélemy"),
    ("BM", "Bermuda"),
    ("BN", "Brunei Darussalam"),
    ("BO", "Bolivia (Plurinational State of)"),
    ("BQ", "Bonaire, Sint Eustatius, Saba"),
    ("BR", "Brazil"),
    ("BS", "Bahamas"),
    ("BT", "Bhutan"),
    ("BV", "Bouvet Island"),
    ("BW", "Botswana"),
    ("BY", "Belarus"),
    ("BZ", "Belize"),
    ("CA", "Canada"),
    ("CC", "Cocos (Keeling) Islands"),
    ("CD", "Congo (the Democratic Republic of)"),
    ("CF", "Central African Republic"),
    ("CG", "Congo"),
    ("CH", "Switzerland"),
    ("CI", "Côte d'Ivoire"),
    ("CK", "Cook Islands"),
    ("CL", "Chile"),
    ("CM", "Cameroon"),
    ("CN", "China"),
    ("CO", "Colombia"),
    ("CR", "Costa Rica"),
    ("CU", "Cuba"),
    ("CV", "Cabo Verde"),
    ("CW", "Curaçao"),
    ("CX", "Christmas Island"),
    ("CY", "Cyprus"),
    ("CZ", "Czechia"),
    ("DE", "Germany"),
    ("DJ", "Djibouti"),
    ("DK", "Denmark"),
    ("DM", "Dominica"),
    ("DO", "Dominican Republic"),
    ("DZ", "Algeria"),
    ("EC", "Ecuador"),
    ("EE", "Estonia"),
    ("EG", "Egypt"),
    ("EH", "Western Sahara"),
    ("ER", "Eritrea"),
    ("ES", "Spain"),
    ("ET", "Ethiopia"),
    ("FI", "Finland"),
    ("FJ", "Fiji"),
    ("FK", "Falkland Islands [Malvinas]"),
    ("FM", "Micronesia (Federated States of)"),
    ("FO", "Faroe Islands"),
    ("FR", "France"),
    ("GA", "Gabon"),
    ("GB", "United Kingdom of Great Britain and Northern Ireland"),
    ("GD", "Grenada"),
    ("GE", "Georgia"),
    ("GF", "French Guiana"),
    ("GG", "Guernsey"),
    ("GH", "Ghana"),
    ("GI", "Gibraltar"),
    ("GL", "Greenland"),
    ("GM", "Gambia"),
    ("GN", "Guinea"),
    ("GP", "Guadeloupe"),
    ("GQ", "Equatorial Guinea"),
    ("GR", "Greece"),
    ("GS", "South Georgia and"),
    ("GT", "Guatemala"),
    ("GU", "Guam"),
    ("GW", "Guinea-Bissau"),
    ("GY", "Guyana"),
    ("HK", "Hong Kong"),
    ("HM", "Heard Island and McDonald Islands"),
    ("HN", "Honduras"),
    ("HR", "Croatia"),
    ("HT", "Haiti"),
    ("HU", "Hungary"),
    ("ID", "Indonesia"),
    ("IE", "Ireland"),
    ("IL", "Israel"),
    ("IM", "Isle of Man"),
    ("IN", "India"),
    ("IO", "British Indian Ocean Territory"),
    ("IQ", "Iraq"),
    ("IR", "Iran (Islamic Republic of)"),
    ("IS", "Iceland"),
    ("IT", "Italy"),
    ("JE", "Jersey"),
    ("JM", "Jamaica"),
    ("JO", "Jordan"),
    ("JP", "Japan"),
    ("KE", "Kenya"),
    ("KG", "Kyrgyzstan"),
    ("KH", "Cambodia"),
    ("KI", "Kiribati"),
    ("KM", "Comoros"),
    ("KN", "Saint Kitts and Nevis"),
    ("KP", "Korea (the Democratic People's Republic of)"),
    ("KR", "Korea (the Republic of)"),
    ("KW", "Kuwait"),
    ("KY", "Cayman Islands"),
    ("KZ", "Kazakhstan"),
    ("LA", "Lao People's Democratic Republic"),
    ("LB", "Lebanon"),
    ("LC", "Saint Lucia"),
    ("LI", "Liechtenstein"),
    ("LK", "Sri Lanka"),
    ("LR", "Liberia"),
    ("LS", "Lesotho"),
    ("LT", "Lithuania"),
    ("LU", "Luxembourg"),
    ("LV", "Latvia"),
    ("LY", "Libya"),
    ("MA", "Morocco"),
    ("MC", "Monaco"),
    ("MD", "Moldova (the Republic of)"),
    ("ME", "Montenegro"),
    ("MF", "Saint Martin (French part)"),
    ("MG", "Madagascar"),
    ("MH", "Marshall Islands"),
    ("MK", "North Macedonia"),
    ("ML", "Mali"),
    ("MM", "Myanmar"),
    ("MN", "Mongolia"),
    ("MO", "Macao"),
    ("MP", "Northern Mariana Islands"),
    ("MQ", "Martinique"),
    ("MR", "Mauritania"),
    ("MS", "Montserrat"),
    ("MT", "Malta"),
    ("MU", "Mauritius"),
    ("MV", "Maldives"),
    ("MW", "Malawi"),
    ("MX", "Mexico"),
    ("MY", "Malaysia"),
    ("MZ", "Mozambique"),
    ("NA", "Namibia"),
    ("NC", "New Caledonia"),
    ("NE", "Niger"),
    ("NF", "Norfolk Island"),
    ("NG", "Nigeria"),
    ("NI", "Nicaragua"),
    ("NL", "Netherlands, Kingdom of"),
    ("NO", "Norway"),
    ("NP", "Nepal"),
    ("NR", "Nauru"),
    ("NU", "Niue"),
    ("NZ", "New Zealand"),
    ("OM", "Oman"),
    ("PA", "Panama"),
    ("PE", "Peru"),
    ("PF", "French Polynesia"),
    ("PG", "Papua New Guinea"),
    ("PH", "Philippines"),
    ("PK", "Pakistan"),
    ("PL", "Poland"),
    ("PM", "Saint Pierre and Miquelon"),
    ("PN", "Pitcairn"),
    ("PR", "Puerto Rico"),
    ("PS", "Palestine, State of"),
    ("PT", "Portugal"),
    ("PW", "Palau"),
    ("PY", "Paraguay"),
    ("QA", "Qatar"),
    ("RE", "Réunion"),
    ("RO", "Romania"),
    ("RS", "Serbia"),
    ("RU", "Russian Federation"),
    ("RW", "Rwanda"),
    ("SA", "Saudi Arabia"),
    ("SB", "Solomon Islands"),
    ("SC", "Seychelles"),
    ("SD", "Sudan"),
    ("SE", "Sweden"),
    ("SG", "Singapore"),
    ("SH", "Saint Helena, Ascension Island, Tristan da Cunha"),
    ("SI", "Slovenia"),
    ("SJ", "Svalbard, Jan Mayen"),
    ("SK", "Slovakia"),
    ("SL", "Sierra Leone"),
    ("SM", "San Marino"),
    ("SN", "Senegal"),
    ("SO", "Somalia"),
    ("SR", "Suriname"),
    ("SS", "South Sudan"),
    ("ST", "Sao Tome and Principe"),
    ("SV", "El Salvador"),
    ("SX", "Sint Maarten (Dutch part)"),
    ("SY", "Syrian Arab Republic"),
    ("SZ", "Eswatini"),
    ("TC", "Turks and Caicos Islands"),
    ("TD", "Chad"),
    ("TF", "French Southern Territories"),
    ("TG", "Togo"),
    ("TH", "Thailand"),
    ("TJ", "Tajikistan"),
    ("TK", "Tokelau"),
    ("TL", "Timor-Leste"),
    ("TM", "Turkmenistan"),
    ("TN", "Tunisia"),
    ("TO", "Tonga"),
    ("TR", "Türkiye"),
    ("TT", "Trinidad and Tobago"),
    ("TV", "Tuvalu"),
    ("TW", "Taiwan (Province of China)"),
    ("TZ", "Tanzania,"),
    ("UA", "Ukraine"),
    ("UG", "Uganda"),
    ("UM", "United States Minor Outlying Islands"),
    ("US", "United States of America"),
    ("UY", "Uruguay"),
    ("UZ", "Uzbekistan"),
    ("VA", "Holy See"),
    ("VC", "Saint Vincent and"),
    ("VE", "Venezuela (Bolivarian Republic of)"),
    ("VG", "Virgin Islands (British)"),
    ("VI", "Virgin Islands (U.S.)"),
    ("VN", "Viet Nam"),
    ("VU", "Vanuatu"),
    ("WF", "Wallis and Futuna"),
    ("WS", "Samoa"),
    ("YE", "Yemen"),
    ("YT", "Mayotte"),
    ("ZA", "South Africa"),
    ("ZM", "Zambia"),
    ("ZW", "Zimbabwe"),
];

#[derive(Debug)]
struct CountryPhoneDescriptor {
    country: String,
    alpha2: String,
    code: String,
    landline_format: String,
    mobile_format: String,
    possible_lengths: Vec<u16>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(METADATA_URL).await?;
    let raw_metadata = response.bytes().await?;
    let raw_metadata = std::str::from_utf8(&raw_metadata)?;
    let cursor = Cursor::new(raw_metadata);
    let metadata = loader::load(BufReader::new(cursor))?;
    // let json_matadata = serde_json::to_string(&metadata)?;
    // std::fs::write("metadata.json", json_matadata)?;

    let mut country_descriptors = indexmap::IndexMap::new();
    metadata.iter().for_each(|country_metadata| {
        if let (Some(country), Some(fixed_line), Some(mobile)) = (
            &country_metadata.id,
            &country_metadata.fixed_line,
            &country_metadata.mobile,
        ) {
            if let Some((_, full_name)) = COUNTRIES.iter().find(|(code, _)| *code == country) {
                let mut possible_lengths = Vec::new();
                possible_lengths.extend(fixed_line.possible_length.clone());
                possible_lengths.extend(fixed_line.possible_local_length.clone());
                possible_lengths.extend(mobile.possible_length.clone());
                possible_lengths.extend(mobile.possible_local_length.clone());
                possible_lengths.sort();
                possible_lengths.dedup();

                country_descriptors.insert(
                    country.to_string(),
                    CountryPhoneDescriptor {
                        country: full_name.to_string(),
                        alpha2: country.to_string(),
                        code: country_metadata.country_code.unwrap().to_string(),
                        landline_format: fixed_line.national_number.as_ref().unwrap().to_string(),
                        mobile_format: mobile.national_number.as_ref().unwrap().to_string(),
                        possible_lengths,
                    },
                );
            }
        };
    });

    country_descriptors.reverse();

    let by_alpha2 =
        country_descriptors
            .iter()
            .fold(String::new(), |mut acc, (_country, descriptor)| {
                acc += "#(";
                acc = acc + "\"" + &descriptor.alpha2 + "\"";

                acc += ",#(";
                acc = acc + "\"" + &descriptor.country + "\",";
                acc = acc + "\"" + &descriptor.code + "\",";

                acc = acc
                    + "\""
                    + &descriptor
                        .mobile_format
                        .replace([' ', '\n', '\t'], "")
                        .replace('\\', "\\\\")
                    + "\",";

                acc = acc
                    + "\""
                    + &descriptor
                        .landline_format
                        .replace([' ', '\n', '\t'], "")
                        .replace('\\', "\\\\")
                    + "\",";

                acc = acc
                    + "["
                    + (descriptor
                        .possible_lengths
                        .iter()
                        .map(|length| length.to_string())
                        .collect::<Vec<String>>()
                        .join(","))
                    .as_str()
                    + "]";

                acc += "),),";
                acc
            });

    let by_code =
        country_descriptors
            .iter()
            .fold(String::new(), |mut acc, (_country, descriptor)| {
                acc += "#(";
                acc = acc + "\"" + &descriptor.code + "\"";
                acc += ",#(";

                acc = acc + "\"" + &descriptor.country + "\",";
                acc = acc + "\"" + &descriptor.alpha2 + "\",";

                acc = acc
                    + "\""
                    + &descriptor
                        .mobile_format
                        .replace([' ', '\n', '\t'], "")
                        .replace('\\', "\\\\")
                    + "\",";
                acc = acc
                    + "\""
                    + &descriptor
                        .landline_format
                        .replace([' ', '\n', '\t'], "")
                        .replace('\\', "\\\\")
                    + "\",";

                acc = acc
                    + "["
                    + (descriptor
                        .possible_lengths
                        .iter()
                        .map(|length| length.to_string())
                        .collect::<Vec<String>>()
                        .join(","))
                    .as_str()
                    + "]";

                acc += "),),";
                acc
            });

    let gleam_map = String::from("//// This file is generated by metadata-provider\n")
        + &String::from("pub const by_alpha2=[")
        + &by_alpha2
        + &String::from("]\n")
        + &String::from("pub const by_code=[")
        + &by_code
        + &String::from("]");

    let mut gleam_file = std::fs::File::create(GLEAM_FILE_PATH)?;
    gleam_file.write_all(gleam_map.as_bytes())?;

    Ok(())
}
