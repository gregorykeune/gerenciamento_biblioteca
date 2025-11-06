use std::fmt;

use crate::traits::Identificavel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct Usuario {
    pub id: Uuid,
    pub nome: String,
}

impl Usuario {
    pub fn new(nome: String) -> Self {
        let usuario = Usuario {
            id: Uuid::new_v4(),
            nome,
        };
        usuario
    }

    pub fn get_nome(&self) -> String {
        self.nome.clone()
    }
}

impl Identificavel for Usuario {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl fmt::Display for Usuario {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Nome: {}", self.nome,)
    }
}
