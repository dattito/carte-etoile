use std::{fs::File, path::Path};

use chrono::Duration;
use openssl::rsa::Rsa;
use passes::{
    barcode::{Barcode, BarcodeFormat},
    fields, resource, semantic_tags,
    sign::{self, SignConfig},
    visual_appearance::{Color, VisualAppearance},
    web_service::WebService,
    Package, PassBuilder, PassConfig,
};

use crate::Result;

#[derive(Clone, Debug)]
pub struct PassMaker {
    team_identifier: String,
    pass_type_identifier: String,
    i_sign_config: ISignConfig,
    web_service_url: String,
    logo_path: String,
    icon_path: String,
}

impl PassMaker {
    pub fn new(
        i_sign_config: ISignConfig,
        team_identifier: String,
        pass_type_identifier: String,
        web_service_url: String,
        logo_path: String,
        icon_path: String,
    ) -> Result<Self> {
        Ok(Self {
            i_sign_config,
            team_identifier,
            pass_type_identifier,
            web_service_url,
            logo_path,
            icon_path,
        })
    }

    pub fn pass_type_identifier(&self) -> &str {
        &self.pass_type_identifier
    }

    pub fn new_pass(&self, serial_number: String, authentication_token: String) -> Result<Package> {
        // Calculate time
        let time_to_departure = chrono::offset::Local::now().to_utc() + Duration::hours(4);
        let time_to_boarding = time_to_departure - Duration::minutes(30);
        let time_to_arrive = time_to_departure + Duration::hours(4);

        // Creating pass
        let pass = PassBuilder::new(PassConfig {
            organization_name: "Datti Railways".into(),
            description: "DRW Boarding Pass".into(),
            pass_type_identifier: self.pass_type_identifier.clone(),
            team_identifier: self.team_identifier.clone(),
            serial_number: serial_number.clone(),
        })
        .appearance(VisualAppearance {
            label_color: Color::white(),
            foreground_color: Color::white(),
            background_color: Color::new(0, 143, 212),
        })
        .fields(
            fields::Type::BoardingPass {
                pass_fields: fields::Fields {
                    ..Default::default()
                },
                transit_type: fields::TransitType::Train,
            }
            .add_primary_field(fields::Content::new(
                "from",
                "OAK",
                fields::ContentOptions {
                    label: String::from("Oak island").into(),
                    ..Default::default()
                },
            ))
            .add_primary_field(fields::Content::new(
                "to",
                "MVK",
                fields::ContentOptions {
                    label: String::from("Маврикий").into(),
                    ..Default::default()
                },
            ))
            .add_auxiliary_field(fields::Content::new(
                "seq",
                "457",
                fields::ContentOptions {
                    label: String::from("seq").into(),
                    ..Default::default()
                },
            ))
            .add_auxiliary_field(fields::Content::new(
                "boards",
                "18:46",
                fields::ContentOptions {
                    label: String::from("scheduled").into(),
                    ..Default::default()
                },
            ))
            .add_auxiliary_field(fields::Content::new(
                "seat",
                "20A",
                fields::ContentOptions {
                    label: String::from("seat").into(),
                    ..Default::default()
                },
            ))
            .add_auxiliary_field(fields::Content::new(
                "group",
                &fastrand::i8(0..100).to_string(),
                fields::ContentOptions {
                    label: String::from("random number").into(),
                    ..Default::default()
                },
            ))
            .add_secondary_field(fields::Content::new(
                "passenger",
                "John Cena",
                fields::ContentOptions {
                    label: String::from("passenger").into(),
                    ..Default::default()
                },
            ))
            .add_header_field(fields::Content::new(
                "gate",
                "21",
                fields::ContentOptions {
                    label: String::from("gate").into(),
                    ..Default::default()
                },
            ))
            .add_header_field(fields::Content::new(
                "flight",
                "DL 1132",
                fields::ContentOptions {
                    label: String::from("flight").into(),
                    ..Default::default()
                },
            ))
            .add_back_field(fields::Content::new(
                "about",
                "This is test boarding pass for Datti Railways.",
                fields::ContentOptions {
                    label: String::from("About").into(),
                    ..Default::default()
                },
            ))
            .add_back_field(fields::Content::new(
                "serial-number",
                &serial_number,
                fields::ContentOptions {
                    label: String::from("Github").into(),
                    ..Default::default()
                },
            )),
        )
        .relevant_date(time_to_departure)
        .expiration_date(time_to_arrive)
        .semantics(semantic_tags::SemanticTags {
            airline_code: String::from("DL 1132").into(),
            departure_gate: String::from("21").into(),
            departure_location: semantic_tags::SemanticTagLocation {
                latitude: 43.3948533,
                longitude: 132.1451673,
            }
            .into(),
            original_boarding_date: time_to_boarding.into(),
            original_departure_date: time_to_departure.into(),
            original_arrival_date: time_to_arrive.into(),
            seats: vec![semantic_tags::SemanticTagSeat {
                seat_identifier: String::from("20A").into(),
                seat_number: String::from("A").into(),
                seat_row: String::from("20").into(),
                seat_type: String::from("econom").into(),
                ..Default::default()
            }],
            ..Default::default()
        })
        .add_barcode(Barcode {
            message: serial_number,
            format: BarcodeFormat::QR,
            ..Default::default()
        })
        .web_service(WebService {
            web_service_url: self.web_service_url.clone(),
            authentication_token,
        })
        .build();

        // Display pass.json
        // let json = pass.make_json().unwrap();
        // info!("pass.json: {}", json);

        // Creating package
        let mut package = Package::new(pass);

        // Adding icon
        let image_path = Path::new(&self.icon_path);
        let file = match File::open(image_path) {
            Err(why) => panic!("couldn't open {}: {}", image_path.display(), why),
            Ok(file) => file,
        };
        package
            .add_resource(resource::Type::Icon(resource::Version::Size2X), file)
            .unwrap();

        // Adding logo
        let image_path = Path::new(&self.logo_path);
        let file = match File::open(image_path) {
            Err(why) => panic!("couldn't open {}: {}", image_path.display(), why),
            Ok(file) => file,
        };

        package
            .add_resource(resource::Type::Logo(resource::Version::Size2X), file)
            .unwrap();

        // Add certificates
        package.add_certificates(self.i_sign_config.new_sign_config()?);

        // Save package as .pkpass
        // let path = Path::new("DAL-boardingpass.pkpass");
        // let file = match File::create(path) {
        //     Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        //     Ok(file) => file,
        // };
        // package.write(file).unwrap();

        Ok(package)
    }
}

#[derive(Clone, Debug)]
pub struct ISignConfig {
    pub sign_cert: Vec<u8>,
    pub sign_key: Vec<u8>,
}

impl ISignConfig {
    pub fn new(cert: &str, key: &str, passphrase: &str) -> Result<Self> {
        let sign_cert_path = Path::new(cert);
        let mut file_sign_cert = File::open(sign_cert_path)?;
        let mut sign_cert_data = Vec::new();
        std::io::Read::read_to_end(&mut file_sign_cert, &mut sign_cert_data)?;

        let sign_cert_key_path = Path::new(key);
        let mut file_sign_key_cert = File::open(sign_cert_key_path)?;

        let mut sign_cert_key_data = Vec::new();
        std::io::Read::read_to_end(&mut file_sign_key_cert, &mut sign_cert_key_data)?;

        let rsa = Rsa::private_key_from_pem_passphrase(&sign_cert_key_data, passphrase.as_bytes())?;
        let decrypted_key_pem = rsa.private_key_to_pem()?;

        Ok(Self {
            sign_cert: sign_cert_data,
            sign_key: decrypted_key_pem,
        })
    }

    fn new_sign_config(&self) -> Result<SignConfig> {
        Ok(SignConfig::new(
            sign::WWDR::G4,
            &self.sign_cert,
            &self.sign_key,
        )?)
    }
}
