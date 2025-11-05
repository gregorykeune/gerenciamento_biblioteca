use chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::biblioteca::{Biblioteca, ErroBiblioteca, emprestimos::{Emprestimo, StatusEmprestimo}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Livro  {
    id: Uuid,
    titulo: String,
    autor: String,
    ano: u16,
    status: StatusLivro,
}

#[derive(Debug, Serialize, Deserialize)]

pub enum StatusLivro {
    Disponivel,
    Emprestado,
}

impl Livro {
    pub fn emprestar(&self, biblioteca: &mut Biblioteca, id_usuario: Uuid) -> Result<(), ErroBiblioteca> {
        // Verifica se o usuário existe
        if !biblioteca.usuarios.contains_key(&id_usuario) {
            return Err(ErroBiblioteca::UsuarioNaoEncontrado);
        }

        // Verifica se o livro ainda existe na biblioteca
        if !biblioteca.livros.contains_key(&self.id) {
            return Err(ErroBiblioteca::LivroNaoEncontrado);
        }

        // Verifica se o livro já está emprestado
        return match &self.status {
            StatusLivro::Emprestado => {Err(ErroBiblioteca::LivroJaEmprestado)}
            StatusLivro::Disponivel => {
                let novo_emprestimo = Emprestimo::novo(self.id, id_usuario);
                biblioteca.emprestimos.insert(novo_emprestimo.id_emprestimo, novo_emprestimo);
                Ok(())
            }
        }


    
    }

    pub fn devolver(emprestimo: &mut Emprestimo) -> Result<(), ErroBiblioteca> {
        return match emprestimo.status {
            StatusEmprestimo::Devolvido => {
                Err(ErroBiblioteca::LivroJaDisponivel)
            }
            StatusEmprestimo::Ativo => {
                emprestimo.status = StatusEmprestimo::Devolvido;    
                emprestimo.data_devolucao = Some(Local::now().date_naive());
                Ok(())
            }
        }
    
    }
}