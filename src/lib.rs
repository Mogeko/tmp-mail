use structopt::StructOpt;
use std::ffi::OsString;


#[derive(StructOpt, Debug)]
#[structopt(name="tmpMail")]
struct Opt {
    #[structopt(short = "p", long = "port", name = "port")]
    prots: Vec<String>,

    #[structopt(short = "n", long = "name", name = "SMTP service name")]
    name: Option<String>,
}

impl Opt {
    pub fn get_prots(&self) -> Vec<String> {
        if self.prots.is_empty() {
            vec!["localhost:25".to_owned()]
        } else {
            self.prots.to_vec()
        }
    }

    pub fn get_name(&self) -> std::io::Result<OsString> {
        match &self.name {
            Some(name) => Ok(OsString::from(name)),
            None       => hostname::get(),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn setup_server() -> Opt {
        Opt {
            prots: vec!["localhost:2020".to_owned()],
            name: Some(String::from("mogeko.me")),
        }
    }

    fn setup_server_default() -> Opt {
        Opt {
            prots: Vec::new(),
            name: None,
        }
    }

    #[test]
    fn test_get_server_prots() {
        let server = setup_server();
        assert_eq!(server.get_prots(), vec!["localhost:2020".to_owned()]);
    }

    #[test]
    fn test_get_server_prots_default() {
        let server = setup_server_default();
        assert_eq!(server.get_prots(), vec!["localhost:25".to_owned()]);
    }

    #[test]
    fn test_get_server_name() {
        let server = setup_server();
        let result = server.get_name();
        assert!(result.is_ok());
        if let Ok(name) = result {
            assert_eq!(name, OsString::from("mogeko.me"))
        }
    }

    #[test]
    fn test_get_server_name_default() {
        let server = setup_server_default();
        let result = server.get_name();
        assert!(result.is_ok());
        if let Ok(test_name) = result {
            if let Ok(right_name) = hostname::get() {
                assert_eq!(test_name, right_name);
            };
        };
    }
}
