use anyhow::Result;
pub mod functions;
mod constants;
use constants::program;

fn main() -> Result<()> {
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]//cargo test decenwser1 -- --show-output
    fn decenwser1() {
        functions::decenwser1::decenwser1(
            &program::program().unwrap(),
        )
        .unwrap();
    }
    #[test]//cargo test main_account -- --show-output
    fn main_account() {
        functions::main_account::main_account(
            &program::program().unwrap(),
            "wifi2".to_string()
        )
        .unwrap();
    }
    #[test]//cargo test html_store -- --show-output
    fn html_store() {
        functions::html_store::html_store(
            &program::program().unwrap(), 
            "<label>Correo electrónico:</label><input type=#~email#~ name=#~email#~><br><label>Mensaje:</label>       <textarea name=#~message#~></textarea><br><input type=#~submit#~ value=#~Enviar#~></form></main><footer> <p>Copyright © 2021 Mi sitio web</p>   </footer>   <script type=#~text/javascript#~ src=#~script.js#~></script> </body> </html>".to_string(),
            "app2".to_string()
        )
        .unwrap();
    }
    #[test]//cargo test css_store -- --show-output
    fn css_store() {
        functions::css_store::css_store(
            &program::program().unwrap(),
            "body {   font-family: Arial#! sans-serif;   margin: 0;   padding: 0; }  header#! footer {   background-color: #333;   color: #fff;   padding: 1em; }  nav ul {   list-style: none;   margin: 0;   padding: 0; }  nav li {   display: inline-block;   margin-right: 1em; }  nav a {   color: #333;   text-decoration: none; }  main {   padding: 1em; }  form label {   display: block;   margin-bottom: 0.5em; }  form input[type=#~submit#~] {   background-color: #333;   color: #fff;   padding: 0.5em;   border: none; }".to_string(),
            "app2".to_string()
        )
        .unwrap();
    }
    #[test]//cargo test js_store -- --show-output
    fn js_store() {
        functions::js_store::js_store(
            &program::program().unwrap(),
            "dsfsdfsdfsd".to_string(),
            "app".to_string()
        )
        .unwrap();
    }
    #[test]//cargo test check -- --show-output
    fn check() {
        functions::check::check(
            "react".to_string(),
        )
        .unwrap();
    }
}