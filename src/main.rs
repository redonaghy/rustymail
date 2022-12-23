use lettre::Message;
use lettre::Transport;
use lettre::SmtpTransport;
use clap::Parser;
use clap::arg;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'f', long = "from")]
    from: String,

    #[arg(short = 't', long = "to")]
    to: String,
    
    #[arg(short = 's', long = "subject")]
    subject: String,
    
    #[arg(short = 'b', long = "body")]
    body: String
}

fn main() {
    let args = Cli::parse();

    let email = Message::builder()
                                .from(args.from.parse().unwrap())
                                .to(args.to.parse().unwrap())
                                .subject(args.subject)
                                .body(args.body)
                                .unwrap();

    let mailer = SmtpTransport::unencrypted_localhost();
    mailer.send(&email).unwrap();
}
