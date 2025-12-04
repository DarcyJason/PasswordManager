use crate::{
    commands::{
        id::parse_id_option, login::parse_login_option, note::parse_note_option,
        payment_card::parse_payment_card_option, ssh_secret_key::parse_ssh_secret_key_option,
    },
    options::{Cli, Commands},
};

pub mod id;
pub mod login;
pub mod note;
pub mod payment_card;
pub mod ssh_secret_key;

pub fn parse_cli(cli: Cli) {
    if let Some(commands) = cli.commands {
        match commands {
            Commands::Login {
                name,
                username,
                password,
                api_key,
                url,
                description,
            } => parse_login_option(name, username, password, api_key, url, description),
            Commands::PaymentCard {
                name,
                owner_name,
                card_number,
                card_brand,
                expired_month,
                expired_year,
                safe_code,
                description,
            } => parse_payment_card_option(
                name,
                owner_name,
                card_number,
                card_brand,
                expired_month,
                expired_year,
                safe_code,
                description,
            ),
            Commands::ID {
                name,
                title,
                first_name,
                middle_name,
                last_name,
                username,
                company,
                social_security_number,
                passport_number,
                license_number,
                email,
                telephone_number,
                address,
                town,
                province,
                postcode,
                nation,
                description,
            } => {
                parse_id_option(
                    name,
                    title,
                    first_name,
                    middle_name,
                    last_name,
                    username,
                    company,
                    social_security_number,
                    passport_number,
                    license_number,
                    email,
                    telephone_number,
                    address,
                    town,
                    province,
                    postcode,
                    nation,
                    description,
                );
            }
            Commands::Note { name, description } => {
                parse_note_option(name, description);
            }
            Commands::SSHSecretKey {
                name,
                private_key,
                public_key,
                fingerprint,
                description,
            } => {
                parse_ssh_secret_key_option(
                    name,
                    private_key,
                    public_key,
                    fingerprint,
                    description,
                );
            }
        }
    }
}
