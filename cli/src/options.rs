use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Password Manager")]
#[command(version = "0.1.0")]
#[command(about = "Store your password or API key safely", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Store your website information")]
    Login {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long)]
        api_key: Option<String>,
        #[arg(long)]
        url: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    #[command(about = "Store your payment card information")]
    PaymentCard {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        owner_name: Option<String>,
        #[arg(long)]
        card_number: Option<String>,
        #[arg(long)]
        card_brand: Option<String>,
        #[arg(long)]
        expired_month: Option<i32>,
        #[arg(long)]
        expired_year: Option<i32>,
        #[arg(long)]
        safe_code: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    #[command(about = "Store ID")]
    ID {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        first_name: Option<String>,
        #[arg(long)]
        middle_name: Option<String>,
        #[arg(long)]
        last_name: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        company: Option<String>,
        #[arg(long)]
        social_security_number: Option<String>,
        #[arg(long)]
        passport_number: Option<String>,
        #[arg(long)]
        license_number: Option<String>,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        telephone_number: Option<String>,
        #[arg(long)]
        address: Option<Vec<String>>,
        #[arg(long)]
        town: Option<String>,
        #[arg(long)]
        province: Option<String>,
        #[arg(long)]
        postcode: Option<String>,
        #[arg(long)]
        nation: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    #[command(about = "Store note")]
    Note {
        name: Option<String>,
        description: Option<String>,
    },
    #[command(about = "Store your SSH Secret Key")]
    SSHSecretKey {
        name: Option<String>,
        private_key: Option<String>,
        public_key: Option<String>,
        fingerprint: Option<String>,
        description: Option<String>,
    },
}
