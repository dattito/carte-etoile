use std::{
    fs::File,
    io::{Cursor, Seek, SeekFrom},
    path::Path,
};

use axum::body::Body;
use chrono::{DateTime, Utc};
use openssl::rsa::Rsa;
use passes::{
    barcode::{Barcode, BarcodeFormat},
    fields::{self, DateStyle},
    resource,
    sign::{self, SignConfig},
    visual_appearance::{Color, VisualAppearance},
    web_service::WebService,
    Package, PassBuilder, PassConfig,
};
use tokio_util::io::ReaderStream;

use crate::{image::ImageMaker, Error, Result};

pub struct LoyalityPass {
    pub already_redeemed: i32,
    pub total_points: i32,
    pub current_points: i32,
    pub pass_holder_name: String,
    pub last_use: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct PassMaker {
    team_identifier: String,
    pass_type_identifier: String,
    i_sign_config: ISignConfig,
    web_service_url: String,
    _logo_path: String,
    icon_path: String,
    image_maker: ImageMaker,
}

impl PassMaker {
    pub fn new(
        i_sign_config: ISignConfig,
        team_identifier: String,
        pass_type_identifier: String,
        web_service_url: String,
        logo_path: String,
        icon_path: String,
        image_maker: ImageMaker,
    ) -> Result<Self> {
        Ok(Self {
            i_sign_config,
            team_identifier,
            pass_type_identifier,
            web_service_url,
            _logo_path: logo_path,
            icon_path,
            image_maker,
        })
    }

    pub fn pass_type_identifier(&self) -> &str {
        &self.pass_type_identifier
    }

    pub fn new_loyality_pass(
        &self,
        serial_number: String,
        authentication_token: String,
        loyality_pass: LoyalityPass,
    ) -> Result<Package> {
        let pass = PassBuilder::new(PassConfig {
            organization_name: "Boulder Bubbletea".into(),
            description: "Boulder Bubbletea Pass".into(),
            pass_type_identifier: self.pass_type_identifier.clone(),
            team_identifier: self.team_identifier.clone(),
            serial_number: serial_number.clone(),
        })
        .appearance(VisualAppearance {
            label_color: Color::white(),
            foreground_color: Color::white(),
            background_color: Color::new(255, 145, 160),
        })
        .set_sharing_prohibited(true)
        .fields({
            let mut f = fields::Type::Coupon {
                pass_fields: fields::Fields::default(),
            }
            .add_header_field(fields::Content::new(
                "name",
                "Boulder Bubbletea",
                fields::ContentOptions {
                    label: "Store".to_string().into(),
                    ..Default::default()
                },
            ))
            .add_secondary_field(fields::Content::new(
                "name",
                &loyality_pass.pass_holder_name,
                fields::ContentOptions {
                    label: "Dieser Pass gehört".to_string().into(),
                    ..Default::default()
                },
            ))
            .add_secondary_field(fields::Content::new(
                "already_redeemed",
                &loyality_pass.already_redeemed.to_string(),
                fields::ContentOptions {
                    label: "Bereits eingelöst".to_string().into(),
                    ..Default::default()
                },
            ))
            .add_back_field(fields::Content::new(
                "serial-number",
                &serial_number,
                fields::ContentOptions {
                    label: String::from("Serial Number").into(),
                    ..Default::default()
                },
            ));

            if let Some(last_use) = loyality_pass.last_use {
                f = f.add_back_field(fields::Content::new(
                    "last_use",
                    &last_use.to_rfc3339(),
                    fields::ContentOptions {
                        label: "Letzte Nutzung".to_string().into(),
                        time_style: DateStyle::Medium.into(),
                        date_style: DateStyle::Medium.into(),
                        ..Default::default()
                    },
                ));
            }
            f
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

        let mut package = Package::new(pass);

        let image_path = Path::new(&self.icon_path);
        let file = match File::open(image_path) {
            Err(why) => panic!("couldn't open {}: {}", image_path.display(), why),
            Ok(file) => file,
        };
        package
            .add_resource(resource::Type::Icon(resource::Version::Size2X), file)
            .unwrap();

        package
            .add_resource(
                resource::Type::Strip(resource::Version::Size2X),
                Cursor::new(self.image_maker.generate_points_image(
                    loyality_pass.total_points.try_into().unwrap(),
                    loyality_pass.current_points.try_into().unwrap(),
                )?),
            )
            .unwrap();

        package.add_certificates(self.i_sign_config.new_sign_config()?);

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

pub fn body_from_package(package: &mut Package) -> Result<Body> {
    let mut buffer = Cursor::new(Vec::new());

    package.write(&mut buffer).map_err(|_| Error::Unknown)?;

    let _ = buffer.seek(SeekFrom::Start(0))?;

    let stream = ReaderStream::new(buffer);
    Ok(Body::from_stream(stream))

    // Save package as .pkpass
    // let path = Path::new("DAL-boardingpass.pkpass");
    // let file = match File::create(path) {
    //     Err(why) => panic!("couldn't create {}: {}", path.display(), why),
    //     Ok(file) => file,
    // };
    // package.write(file).unwrap();
}
