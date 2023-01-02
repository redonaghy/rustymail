use clap::Parser;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;
use lettre::message::Mailbox;

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

/// Sends an email, returns false if operation fails and prints an error message else returns true.
fn send_email(f: String, t: String, s: String, b: String) -> bool {
    let from_result = f.parse();
    let from = match from_result {
        Ok(f_add) => f_add,
        Err(f_err) => {
            println!("Error Sending Email - Invalid Sender Address:  {}", f_err);
            return false;
        }
    };
    
    let to_result = t.parse();
    let to = match to_result {
        Ok(t_add) => t_add,
        Err(t_err) => {
            println!("Error Sending Email - Invalid Receiver Address:  {}", t_err);
            return false;
        }
    };

    let email_result = Message::builder()
                        .from(Mailbox::new(None, from))
                        .to(Mailbox::new(None, to))
                        .subject(s)
                        .body(b);

    let email = match email_result {
        Ok(em) => em,
        Err(ee) => {
            println!("Error Sending Email - Email Build Error:  {}", ee);
            return false;
        }
    };

    let sender = SmtpTransport::unencrypted_localhost();
    let res = sender.send(&email);

    if res.is_err() {
        println!("Unable To Send Email:  {}", res.unwrap_err());
        return false;
    }

    println!("Email Sent Successfully!");
    return true;
}

fn main() {
    let args = Cli::parse();
    send_email(args.from, args.to, args.subject, args.body);
}
