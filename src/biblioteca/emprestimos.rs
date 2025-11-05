use std::ptr::null;

use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::biblioteca::{Biblioteca, ErroBiblioteca};

pub enum StatusEmprestimo {
    pub Ativo,
    pub Devolvido,
}

pub struct Emprestimo {
    pub id_emprestimo: Uuid,
    pub id_livro: Uuid,
    pub id_usuario: Uuid,
    pub data_emprestimo: NaiveDate,
    pub data_devolucao: Option<NaiveDate>,
    pub status: StatusEmprestimo,
}


impl Emprestimo {
    pub fn novo(id_usuario: Uuid, id_livro: Uuid) -> Self {
        let emprestimo = Emprestimo {
            id_emprestimo: Uuid::new_v4(),
            id_livro: id_livro,
            id_usuario: id_usuario,
            data_emprestimo: Local::now().date_naive(),
            data_devolucao: None,
            status: StatusEmprestimo::Ativo,
        }
    }
}

