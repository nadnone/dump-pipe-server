pub enum Errors {
    FATAL,
    IOLOGSFATAL,
    WARN,
}

impl Errors {
    
    pub fn to_str(&self) -> &str {

        let str;
        match self {
            Errors::FATAL => str = "[!] ERREUR FATALE",
            Errors::WARN => str = "[!] ATTENTION !",
            Errors::IOLOGSFATAL => str = "[!] ERREUR FATALE : Impossible d'écrire ou de lire les logs",
        }
        
        return str;
    }
}