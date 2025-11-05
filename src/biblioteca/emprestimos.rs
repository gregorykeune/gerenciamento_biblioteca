use std::fmt;

use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::traits::Identificavel;


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
    data_devolucao: Option<NaiveDate>,
    pub(crate) status: StatusEmprestimo,
}


impl Emprestimo {
    pub fn new(id_usuario: Uuid, id_livro: Uuid) -> Self {
        let emprestimo = Emprestimo {
            id_emprestimo: Uuid::new_v4(),
            id_livro: id_livro,
            id_usuario: id_usuario,
            data_emprestimo: Local::now().date_naive(),
            data_devolucao: None,
            status: StatusEmprestimo::Ativo,
        };
        emprestimo
    }

    pub fn set_data_devolucao(&mut self, data: NaiveDate) {
        self.data_devolucao = Some(data);
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
            self.data_devolucao
                .map(|d| d.to_string())
                .unwrap_or("Não devolvido".into()),
            match self.status {
                StatusEmprestimo::Ativo => "Livro emprestado",
                StatusEmprestimo::Devolvido => "Livro ja devolvido",
            }
        )
    }
}
