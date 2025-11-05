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
    pub fn emprestar(&self, biblioteca: &mut Biblioteca, id_usuario: Uuid) -> Result<(), ErroBiblioteca> {
        // Verifica se o usuário existe
        if !biblioteca.usuarios.contains_key(&id_usuario) {
            return Err(ErroBiblioteca::UsuarioNaoEncontrado(id_usuario));
        }

        // Verifica se o livro ainda existe na biblioteca
        if !biblioteca.livros.contains_key(&self.id) {
            return Err(ErroBiblioteca::LivroNaoEncontrado(self.id()));
        }

        // Verifica se o livro já está emprestado
        return match &self.status {
            StatusLivro::Emprestado => {Err(ErroBiblioteca::EstadoInvalido("Livro já está emprestado! ".to_string()))}
            StatusLivro::Disponivel => {
                let novo_emprestimo = Emprestimo::new(self.id, id_usuario);
                biblioteca.emprestimos.insert(novo_emprestimo.id(), novo_emprestimo);
                Ok(())
            }
        }    
    }

    pub fn devolver(emprestimo: &mut Emprestimo) -> Result<(), ErroBiblioteca> {
        return match emprestimo.status {
            StatusEmprestimo::Devolvido => {
                Err(ErroBiblioteca::EstadoInvalido("Livro ja está disponivel para emprestimo".to_string()))
            }
            StatusEmprestimo::Ativo => {
                emprestimo.status = StatusEmprestimo::Devolvido;    
                emprestimo.set_data_devolucao(Local::now().date_naive());
                Ok(())
            }
        }
    }

    pub fn new(titulo: String, autor: String, ano: u16) -> Self {
        let livro = Livro {
            id: Uuid::new_v4(),
            titulo,
            autor,
            ano,
            status: StatusLivro::Disponivel,
        };  
        livro
    }

    pub fn get_titulo(self) -> &String {
        &self.titulo
    }
}
