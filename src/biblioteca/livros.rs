use std::fmt;

use chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::biblioteca::{Biblioteca, emprestimos::{Emprestimo, StatusEmprestimo}};
use crate::traits::Identificavel;
use crate::errors::ErroBiblioteca;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Livro  {
    id: Uuid,
    titulo: String,
    autor: String,
    ano: u16,
    status: StatusLivro,
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

    pub fn emprestar(&mut self) -> Result<(), ErroBiblioteca> {
        match &self.status {
            StatusLivro::Emprestado => {
                Err(ErroBiblioteca::EstadoInvalido(
                    "Livro já está emprestado!".to_string()
                ))
            }
            StatusLivro::Disponivel => {
                self.status = StatusLivro::Emprestado;
                Ok(())
            }
        }
    }

    pub fn devolver(&mut self) -> Result<(), ErroBiblioteca> {
        match self.status {
            StatusLivro::Disponivel => {
                Err(ErroBiblioteca::EstadoInvalido(
                    "Livro já está disponível!".to_string()
                ))
            }
            StatusLivro::Emprestado => {
                self.status = StatusLivro::Disponivel;
                Ok(())
            }
        }
    }

    pub fn esta_disponivel(&self) -> bool {
        matches!(self.status, StatusLivro::Disponivel)
    }

    pub fn get_titulo(&self) -> &String {
        &self.titulo
    }

    pub fn get_autor(&self) -> &String {
        &self.autor
    }

    pub fn get_ano(&self) -> u16 {
        self.ano
    }
}
