use anchor_client::{
    anchor_lang::solana_program::hash::hash,
    solana_sdk::{pubkey::Pubkey, signature::Keypair},
    Client, Cluster, Program,
};
use anyhow::{Ok, Result};
use std::rc::Rc;
use std::str::FromStr;
use decenwser::state::{MainAccount, HTML, CSS, JS};
use crate::constants::program_id;

#[derive(Debug)]
pub struct App {
    pub html: String,
    pub css: String,
    pub js: String
}

pub fn check(domain: String) -> Result<()> {
    let program_id: Pubkey = Pubkey::from_str(&program_id::ID).unwrap();
    // pda is set to the public key of the program derived address (PDA) for the domain
    let (pda, _bump) = Pubkey::find_program_address(&[&hash(domain.as_bytes()).to_bytes()], &program_id);
    // A new client is created, connected to the Devnet cluster and using a new keypair
    let client: Client = Client::new(Cluster::Devnet, Rc::new(Keypair::new()));
    // The program is loaded from the client using the program ID
    let program: Program = client.program(program_id);
    let app_data: MainAccount = program.account(pda)?;
    // The following variables are initialized to empty strings and zero values
    let mut html_content: String = "".to_string();
    let mut css_content: String = "".to_string();
    let mut js_content: String = "".to_string();
    let mut html_iter: usize = 0;
    let mut css_iter: usize = 0;
    let mut js_iter: usize = 0;
    // The lengths of the html, css, and js strings are determined
    let html_len: usize = app_data.html.len();
    let css_len: usize = app_data.css.len();
    let js_len: usize = app_data.js.len();
    // The following loops iterate through the html, css, and js strings
    // and send requests to retrieve the content for each section
    while html_iter < html_len {
        let domain_html = domain.clone();
        html_iter += 1;
        // The html_request function is called and the result is added to the html_content string
        html_content += &html_request(html_iter, domain_html)?
            .replace("#~", "\"")
            .replace("#&", "\\")
            .replace("#!", ",");
    }
    while css_iter < css_len {
        let domain_css = domain.clone();
        css_iter += 1;
        // The css_request function is called and the result is added to the css_content string
        css_content += &css_request(css_iter, domain_css)?
            .replace("#~", "\"")
            .replace("#&", "\\")
            .replace("#!", ",");
    }
    while js_iter < js_len {
        let domain_js = domain.clone();
        js_iter += 1;
        // The js_request function is called and the result is added to the js_content string
        js_content += &js_request(js_iter, domain_js)?
            .replace("#~", "\"")
            .replace("#&", "\\")
            .replace("#!", ",");
    }
    // The app struct is created with the retrieved html, css, and js content
    let app: App = App {
        html: html_content,
        css: css_content,
        js: js_content
    };
    print!("{:?}", app);// The app struct is printed to the console
    Ok(())
}

pub fn html_request(html_iter: usize, domain: String) -> Result<String> {
    // We start by getting the ID of the program we want to interact with.
    let program_id: Pubkey = Pubkey::from_str(&program_id::ID).unwrap();
    // Then we create a new client object to interact with the Solana blockchain
    let client: Client = Client::new(Cluster::Devnet, Rc::new(Keypair::new()));
    // We use the client to get a handle to the program we want to interact with.
    let program: Program = client.program(program_id);
    // Next, we use the domain string to calculate a public key for the program-derived account (PDA)
    let (pda, _bump) = Pubkey::find_program_address(&[&hash(domain.as_bytes()).to_bytes()], &program_id);
    let app_data: MainAccount = program.account(pda)?;
    let (html_pbk, _bump) = Pubkey::find_program_address(&[&app_data.html[html_iter - 1].to_le_bytes()], &program_id);
    let html: HTML = program.account(html_pbk)?;
    Ok(html.html)// Finally, we return the HTML data from the retrieved account.
}
pub fn css_request(css_iter: usize, domain: String) -> Result<String> {
    // Get the ID of the program we'll be interacting with
    let program_id: Pubkey = Pubkey::from_str(&program_id::ID).unwrap();
    let client: Client = Client::new(Cluster::Devnet, Rc::new(Keypair::new())); // Create a client for the Solana blockchain
    let program: Program = client.program(program_id); // Get a handle to the program we're interested in
    // Generate a program-derived address (PDA) based on the domain name, using the program ID as a seed
    let (pda, _bump) = Pubkey::find_program_address(&[&hash(domain.as_bytes()).to_bytes()], &program_id);
    let app_data: MainAccount = program.account(pda)?;
    // Generate a PDA for the requested CSS resource, using the program ID as a seed
    let (css_pbk, _bump) = Pubkey::find_program_address(&[&app_data.css[css_iter - 1].to_le_bytes()], &program_id);
    // Retrieve the CSS resource associated with the CSS PDA
    let css: CSS = program.account(css_pbk)?;
    Ok(css.css)// Return the CSS code as a String
}
pub fn js_request(js_iter: usize, domain: String) -> Result<String> {
    // Define a variable called program_id and set it to the ID of a program
    let program_id: Pubkey = Pubkey::from_str(&program_id::ID).unwrap();
    // Create a new client object that connects to the Devnet cluster
    let client: Client = Client::new(Cluster::Devnet, Rc::new(Keypair::new()));
    let program: Program = client.program(program_id); // Get the program associated with the program_id
    let (pda, _bump) = Pubkey::find_program_address(&[&hash(domain.as_bytes()).to_bytes()], &program_id);
    // Get the data associated with the main account associated with the PDA
    let app_data: MainAccount = program.account(pda)?;
    // Calculate the public key of the JavaScript-derived account (JS) and its bump seed
    let (js_pbk, _bump) =
        Pubkey::find_program_address(&[&app_data.js[js_iter - 1].to_le_bytes()], &program_id);
    let js: JS = program.account(js_pbk)?;
    Ok(js.js)// Return the JavaScript code stored in the JS account
}
