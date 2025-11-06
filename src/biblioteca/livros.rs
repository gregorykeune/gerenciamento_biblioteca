use std::fmt;

use crate::traits::Identificavel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Livro {
    id: Uuid,
    titulo: String,
    autor: String,
    ano: u16,
    pub status: StatusLivro,
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub enum StatusLivro {
    Disponivel,
    Emprestado,
}

impl Identificavel for Livro {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl fmt::Display for Livro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Título: {}\nAutor: {}\nAno: {}\nDisponível: {}",
            self.titulo,
            self.autor,
            self.ano,
            match self.status {
                StatusLivro::Disponivel => "Sim",
                StatusLivro::Emprestado => "Não",
            }
        )
    }
}

impl Livro {
    pub fn new(titulo: String, autor: String, ano: u16) -> Self {
        Livro {
            id: Uuid::new_v4(),
            titulo,
            autor,
            ano,
            status: StatusLivro::Disponivel,
        }
    }

    pub fn get_titulo(&self) -> &String {
        &self.titulo
    }

    pub fn get_autor(&self) -> &String {
        &self.autor
    }
}
