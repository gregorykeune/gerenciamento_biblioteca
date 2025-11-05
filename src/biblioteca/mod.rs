pub mod livros;
pub mod emprestimos;
pub mod usuarios;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use livros::*;

#[derive(Debug)]
pub enum ErroBiblioteca {
    LivroNaoEncontrado,
    UsuarioNaoEncontrado,
    EmprestimoNaoEncontrado,
    LivroJaEmprestado,
    LivroJaDisponivel,
    ErroPersistencia(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct DadosPersistencia {
    livros: HashMap<Uuid, Livro>,
    usuarios: HashMap<Uuid, Usuario>,
    emprestimos: HashMap<Uuid, Emprestimo>,
}

struct Biblioteca {
    livros: HashMap<Uuid, Livro>,
    usuarios: HashMap<Uuid, Usuario>,
    emprestimos: HashMap<Uuid, Emprestimo>,
    caminho_arquivo: PathBuf,
}

impl Biblioteca {
    pub fn nova<P: AsRef<Path>>(caminho: P) -> Self {
        Self {
            livros: HashMap::new(),
            usuarios: HashMap::new(),
            emprestimos: HashMap::new(),
            caminho_arquivo: caminho.as_ref().to_path_buf(),
        }
    }

    pub fn carregar<P: AsRef<Path>>(caminho: P) -> Result<Self, ErroBiblioteca> {
        let caminho_path = caminho.as_ref().to_path_buf();
        
        if !caminho_path.exists() {
            return Ok(Self::nova(caminho_path));
        }

        let arquivo = File::open(&caminho_path)
            .map_err(|e| ErroBiblioteca::ErroPersistencia(format!("Erro ao abrir arquivo: {}", e)))?;
        
        let leitor = BufReader::new(arquivo);
        let dados: DadosPersistencia = serde_json::from_reader(leitor)
            .map_err(|e| ErroBiblioteca::ErroPersistencia(format!("Erro ao deserializar JSON: {}", e)))?;

        Ok(Self {
            livros: dados.livros,
            usuarios: dados.usuarios,
            emprestimos: dados.emprestimos,
            caminho_arquivo: caminho_path,
        })
    }

    pub fn salvar(&self) -> Result<(), ErroBiblioteca> {
        let dados = DadosPersistencia {
            livros: self.livros.clone(),
            usuarios: self.usuarios.clone(),
            emprestimos: self.emprestimos.clone(),
        };

        let arquivo = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.caminho_arquivo)
            .map_err(|e| ErroBiblioteca::ErroPersistencia(format!("Erro ao criar arquivo: {}", e)))?;

        let escritor = BufWriter::new(arquivo);
        serde_json::to_writer_pretty(escritor, &dados)
            .map_err(|e| ErroBiblioteca::ErroPersistencia(format!("Erro ao serializar JSON: {}", e)))?;

        Ok(())
    }
}