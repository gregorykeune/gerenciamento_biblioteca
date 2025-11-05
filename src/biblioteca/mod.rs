pub mod livros;
pub mod emprestimos;
pub mod usuarios;

use std::{
    collections::HashMap, fs::{File, OpenOptions}, io::{self, BufReader, BufWriter}, path::{Path, PathBuf}, sync::WaitTimeoutResult
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use livros::*;

use crate::{biblioteca::{emprestimos::Emprestimo, usuarios::Usuario}, traits::Identificavel};
use crate::errors::ErroBiblioteca;

#[derive(Debug, Serialize, Deserialize)]
struct DadosPersistencia {
    livros: HashMap<Uuid, Livro>,
    usuarios: HashMap<Uuid, Usuario>,
    emprestimos: HashMap<Uuid, Emprestimo>,
}

pub struct Biblioteca {
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

    pub fn adicionar_livro(&mut self) -> Result<(), ErroBiblioteca> {
        let titulo = ler_string("Titulo: ".to_string()); 
        let autor = ler_string("Autor: ".to_string()); 
        let ano = ler_u16("Ano: ".to_string()); 

        let livro = Livro::new(titulo, autor, ano);

        self.livros.insert(livro.id(), livro);

        Ok(())
    }


    pub fn adicionar_usuario(&mut self) -> Result<(), ErroBiblioteca> {
        let nome = ler_string("Nome: ".to_string());

        let usuario = Usuario::new(nome);

        self.usuarios.insert(usuario.id, usuario);

        Ok(())
    }

    pub fn listar_livros(&self) {
        if self.livros.is_empty() {
            println!("Nenhum livro cadastrado.");
            return;
        }

        println!("\n=== Lista de Livros ===");
        for (id, livro) in &self.livros {
            println!("ID: {}", id);
            println!("{}", livro); // usa Display de Livro
            println!("---------------------------");
        }
    }

    pub fn listar_usuarios(&self) {
        if self.usuarios.is_empty() {
            println!("Nenhum usuário cadastrado.");
            return;
        }

        println!("\n=== Lista de Usuários ===");
        for (id, usuario) in &self.usuarios {
            println!("ID: {}", id);
            println!("{}", usuario); // usa Display de Usuario
            println!("---------------------------");
        }
    }

    pub fn listar_emprestimos(&self) {
        if self.emprestimos.is_empty() {
            println!("Nenhum empréstimo registrado.");
            return;
        }

        println!("\n=== Lista de Empréstimos ===");
        for (id, emprestimo) in &self.emprestimos {
            println!("ID: {}", id);
            println!("{}", emprestimo); // usa Display de Emprestimo
            println!("---------------------------");
        }
    }

    pub fn buscar_livro_por_titulo(&self, titulo_livro: &str) -> HashMap<Uuid, &Livro> {
        let mut encontrados = HashMap::new();

        for (id_livro, livro) in &self.livros {
            if livro.get_titulo().eq_ignore_ascii_case(titulo_livro) {
                encontrados.insert(*id_livro, livro);
            }
        }

        encontrados
    }
}

pub fn ler_string(mensagem: String) -> String {
    let mut entrada = String::new();

    println!("{}", mensagem);

    io::stdin()
        .read_line(&mut entrada)     // Lê até o usuário apertar Enter
        .expect("Falha ao ler entrada");

    entrada
}

pub fn ler_u16(mensagem: String) -> u16 {
    let mut entrada = String::new();

    println!("{}", mensagem);

    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");

    let numero: u16 = entrada
        .trim() // remove \n e espaços
        .parse() // tenta converter para número
        .expect("Digite um número válido!");

    numero
}