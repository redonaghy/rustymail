extern crate imap;
extern crate native_tls;

use clap::Parser;
use lettre::Address;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;

// #[derive(Parser)]
// struct Cli {
//     #[arg(short = 'f', long = "from")]
//     from: &str,

//     #[arg(short = 't', long = "to")]
//     to: String,
    
//     #[arg(short = 's', long = "subject")]
//     subject: String,
    
//     #[arg(short = 'b', long = "body")]
//     body: String
// }

fn build_transport(user: String, passwd: String, port: u16) -> SmtpTransport {
    let creds = Credentials::new(user, passwd);

    return SmtpTransport::builder_dangerous("localhost").port(port).credentials(creds).build();
}

fn fetch_inbox_top(domain: &str, port: u16, login: String, passwd: String) -> imap::error::Result<Option<String>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect((domain, port), domain, &tls).unwrap();

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login(login, passwd)
        .map_err(|e| e.0)?;

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.fetch("1", "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };

    // extract the message's body
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8!")
        .to_string();

    // be nice to the server and log out
    imap_session.logout()?;

    Ok(Some(body))
}

/// Sends an email, returns false if operation fails and prints an error message else returns true.
fn send_email(f: &str, passwd: String, t: String, s: String, b: String, port: u16) -> bool {
    let from_result = f.parse();
    let from: Address = match from_result {
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

    let sender = build_transport(f.to_string(), passwd, port);
    let res = sender.send(&email);

    if res.is_err() {
        println!("Unable To Send Email:  {}", res.unwrap_err());
        return false;
    }

    println!("Email Sent Successfully!");
    return true;
}

fn main() {
    // let args = Cli::parse();
    // send_email("somebody@localhost", "doggy".to_string(), 
    // "nobody@localhost".to_string(), "Morning!".to_string(), 
    // "I'm sleepy!".to_string(), 3025);

    match fetch_inbox_top("localhost", 993, 
    "nobody@localhost".to_string(), "catto".to_string()) {
        Ok(m) => println!("Email Pulled Successfully!  {}", m.unwrap()),
        Err(e) => println!(":( {}", e)
    }
}
