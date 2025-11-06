use std::fmt;

use crate::traits::Identificavel;
use chrono::{Duration, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]

pub enum StatusEmprestimo {
    Ativo,
    Devolvido,
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct Emprestimo {
    id_emprestimo: Uuid,
    id_livro: Uuid,
    id_usuario: Uuid,
    data_emprestimo: NaiveDate,
    data_devolucao: NaiveDate,
    pub status: StatusEmprestimo,
}

impl Emprestimo {
    pub fn new(id_usuario: Uuid, id_livro: Uuid) -> Self {
        let emprestimo = Emprestimo {
            id_emprestimo: Uuid::new_v4(),
            id_livro: id_livro,
            id_usuario: id_usuario,
            data_emprestimo: Local::now().date_naive(),
            data_devolucao: Local::now().date_naive() + Duration::days(14),
            status: StatusEmprestimo::Ativo,
        };
        emprestimo
    }

    pub fn get_data_devolucao(&self) -> NaiveDate {
        self.data_devolucao
    }

    pub fn get_id_livro(&self) -> Uuid {
        self.id_livro
    }

    pub fn get_id_usuario(&self) -> Uuid {
        self.id_usuario
    }
}

impl Identificavel for Emprestimo {
    fn id(&self) -> Uuid {
        self.id_emprestimo
    }
}

impl fmt::Display for Emprestimo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Livro: {}\nUsuário: {}\nData de Empréstimo: {}\nData de Devolução: {}\nStatus: {}",
            self.id_livro,
            self.id_usuario,
            self.data_emprestimo,
            self.data_devolucao,
            match self.status {
                StatusEmprestimo::Ativo => "Livro emprestado",
                StatusEmprestimo::Devolvido => "Livro ja devolvido",
            }
        )
    }
}
