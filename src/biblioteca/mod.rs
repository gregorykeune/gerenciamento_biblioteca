pub mod emprestimos;
pub mod livros;
pub mod usuarios;

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter},
    path::{Path, PathBuf},
};
use uuid::Uuid;

use livros::*;

use crate::errors::ErroBiblioteca;
use crate::{
    biblioteca::{
        emprestimos::{Emprestimo, StatusEmprestimo},
        usuarios::Usuario,
    },
    traits::Identificavel,
};

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

        let arquivo = File::open(&caminho_path).map_err(|e| {
            ErroBiblioteca::ErroPersistencia(format!("Erro ao abrir arquivo: {}", e))
        })?;

        let leitor = BufReader::new(arquivo);
        let dados: DadosPersistencia = serde_json::from_reader(leitor).map_err(|e| {
            ErroBiblioteca::ErroPersistencia(format!("Erro ao deserializar JSON: {}", e))
        })?;

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
            .map_err(|e| {
                ErroBiblioteca::ErroPersistencia(format!("Erro ao criar arquivo: {}", e))
            })?;

        let escritor = BufWriter::new(arquivo);
        serde_json::to_writer_pretty(escritor, &dados).map_err(|e| {
            ErroBiblioteca::ErroPersistencia(format!("Erro ao serializar JSON: {}", e))
        })?;

        Ok(())
    }

    pub fn emprestar(
        livro: &mut Livro,
        id_usuario: Uuid,
        id_livro: Uuid,
    ) -> Result<Emprestimo, ErroBiblioteca> {
        match livro.status {
            StatusLivro::Emprestado => Err(ErroBiblioteca::EstadoInvalido(
                "Livro já está emprestado!".to_string(),
            )),
            StatusLivro::Disponivel => {
                livro.status = StatusLivro::Emprestado;
                let emprestimo = Emprestimo::new(id_usuario, id_livro);
                Ok(emprestimo)
            }
        }
    }

    // pub fn devolver(
    //     &mut self,
    //     livro: &mut Livro,
    //     emprestimo: &mut Emprestimo,
    // ) -> Result<(), ErroBiblioteca> {
    //     match livro.status {
    //         StatusLivro::Disponivel => Err(ErroBiblioteca::EstadoInvalido(
    //             "Livro já está disponível!".to_string(),
    //         )),
    //         StatusLivro::Emprestado => {
    //             livro.status = StatusLivro::Disponivel;
    //             emprestimo.status = StatusEmprestimo::Devolvido;

    //             Ok(())
    //         }
    //     }
    // }

    pub fn adicionar_livro(&mut self) -> Result<Uuid, ErroBiblioteca> {
        let titulo = ler_string("Titulo: ".to_string());
        let autor = ler_string("Autor: ".to_string());
        let ano = ler_u16("Ano: ".to_string());

        let livro = Livro::new(titulo, autor, ano);
        let id = livro.id();

        self.livros.insert(livro.id(), livro);

        Ok(id)
    }

    pub fn adicionar_usuario(&mut self) -> Result<Uuid, ErroBiblioteca> {
        let nome = ler_string("Nome: ".to_string());
        let usuario = Usuario::new(nome);

        let id = usuario.id;

        self.usuarios.insert(id, usuario);
        Ok(id)
    }

    pub fn registrar_emprestimo(&mut self) -> Result<(), ErroBiblioteca> {
        let mensagem = String::from(
            "Como deseja buscar o livro: 
        [1] Buscar pelo ID (Listar todos os Livros) 
        [2] Buscar pelo Título 
        [3] Buscar pelo Nome do Autor 
        Opção: ",
        );

        self.listar_usuarios();

        let id_usuario = ler_uuid("Digite o UUID do usuário que vai fazer o empréstimo: ");
        println!("***** Realizar Empréstimo ******");

        let opcao = ler_i32(mensagem);

        if opcao == 1 {
            // ======== BUSCA POR ID (LISTAR TODOS OS LIVROS) ========

            // Coleta todos os livros em um vetor para exibir e acessar por índice
            let mut livros_vec: Vec<(&Uuid, &mut Livro)> = self.livros.iter_mut().collect();

            // Exibe a lista com índices numéricos
            Biblioteca::listar_livros_vec(&livros_vec);

            // Usuário escolhe o livro pelo índice mostrado na lista
            let id_livro =
                ler_i32("Digite o ID do livro que deseja pegar emprestado: ".to_string()) as usize;

            // Verifica se o índice é válido
            if id_livro >= livros_vec.len() {
                println!("ID inválido!");
                return Ok(());
            }

            // Obtém o UUID e referência mutável do livro escolhido
            let (uuid_livro, livro_mut) = &mut livros_vec[id_livro];
            let uuid_livro = **uuid_livro; // desreferencia o ponteiro duplo para copiar o UUID

            // Cria o empréstimo
            match Biblioteca::emprestar(livro_mut, id_usuario, uuid_livro) {
                Ok(emprestimo) => {
                    self.emprestimos.insert(emprestimo.id(), emprestimo);
                    println!("✅ Empréstimo registrado com sucesso!");
                }
                Err(e) => println!("❌ Erro: {}", e),
            }
        } else if opcao == 2 {
            let titulo = ler_string("Nome do titulo que deseja buscar: ".to_string());

            if let Some(livros) = self.buscar_livro_por_titulo(&titulo) {
                let id_livro =
                    ler_i32("Confirme o ID do livro que deseja pegar emprestado: ".to_string())
                        as usize;

                if id_livro >= livros.len() {
                    println!("ID inválido!");
                    return Ok(());
                }

                let (uuid_livro, _) = &livros[id_livro];
                let uuid_livro = *uuid_livro; // copia o UUID (tipo Copy)

                if let Some(livro_mut) = self.livros.get_mut(&uuid_livro) {
                    let emprestimo = Biblioteca::emprestar(livro_mut, id_usuario, uuid_livro)?;
                    livro_mut.status = StatusLivro::Emprestado;
                    self.emprestimos.insert(emprestimo.id(), emprestimo);
                }
            } else {
                println!("Nenhum livro encontrado para o autor '{}'.", titulo);
            }
        } else if opcao == 3 {
            // ======== BUSCA POR AUTOR ========
            let nome_autor = ler_string("Nome do autor que deseja buscar: ".to_string());

            if let Some(livros) = self.buscar_livro_por_autor(&nome_autor) {
                let id_livro =
                    ler_i32("Confirme o ID do livro que deseja pegar emprestado: ".to_string())
                        as usize;

                if id_livro >= livros.len() {
                    println!("ID inválido!");
                    return Ok(());
                }

                let (uuid_livro, _) = &livros[id_livro];
                let uuid_livro = *uuid_livro; // copia o UUID (tipo Copy)

                if let Some(livro_mut) = self.livros.get_mut(&uuid_livro) {
                    let emprestimo = Biblioteca::emprestar(livro_mut, id_usuario, uuid_livro)?;
                    livro_mut.status = StatusLivro::Emprestado;
                    self.emprestimos.insert(emprestimo.id(), emprestimo);
                }
            } else {
                println!("Nenhum livro encontrado para o autor '{}'.", nome_autor);
            }
        } else {
            println!("Opção inválida.");
        }

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
            println!("{}", livro);
            println!("---------------------------");
        }
    }

    pub fn listar_livros_vec(livros: &Vec<(&Uuid, &mut Livro)>) {
        if livros.is_empty() {
            println!("Nenhum livro cadastrado.");
            return;
        }

        let mut id = 0;
        println!("\n=== Lista de Livros ===");
        for (_, livro) in livros {
            println!("ID: {}", id);
            println!("{}", livro);
            println!("---------------------------");
            id += 1;
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
            println!("=== Detalhes do Empréstimo ===");
            println!("ID: {}", id);
            self.exibir_emprestimo(emprestimo); // usa Display de Emprestimo
            println!("---------------------------");
        }
    }

    pub fn exibir_emprestimo(&self, emprestimo: &Emprestimo) {
        let livro = self.livros.get(&emprestimo.get_id_livro());
        let usuario = self.usuarios.get(&emprestimo.get_id_usuario());

        match livro {
            Some(l) => println!("Livro: {}", l.get_titulo()),
            None => println!("Livro não encontrado."),
        }
        match usuario {
            Some(u) => println!("Usuário: {}", u.get_nome()),
            None => println!("Usuário não encontrado."),
        }

        println!("Data do Empréstimo: {}", emprestimo.get_data_devolucao());
        println!("Data de Devolução: {}", emprestimo.get_data_devolucao());
        println!(
            "Status: {}",
            match emprestimo.status {
                StatusEmprestimo::Ativo => "Ativo",
                StatusEmprestimo::Devolvido => "Devolvido",
            }
        );
    }

    pub fn buscar_livro_por_titulo(&self, titulo_livro: &str) -> Option<Vec<(Uuid, &Livro)>> {
        let encontrados: Vec<_> = self
            .livros
            .iter()
            .filter(|(_, livro)| livro.get_titulo() == titulo_livro)
            .map(|(id, livro)| (*id, livro))
            .collect();

        if encontrados.is_empty() {
            println!("Nenhum livro encontrado para o título '{}'.", titulo_livro);
            None
        } else {
            println!("\n=== Lista de Livros ===");
            for (i, (_, livro)) in encontrados.iter().enumerate() {
                println!("ID: {}", i);
                println!("{}", livro);
                println!("---------------------------");
            }
            Some(encontrados)
        }
    }

    pub fn buscar_livro_por_autor(&self, autor_livro: &str) -> Option<Vec<(Uuid, &Livro)>> {
        let mut encontrados: Vec<(Uuid, &Livro)> = Vec::new();

        for (id_livro, livro) in &self.livros {
            if livro.get_autor() == autor_livro {
                encontrados.push((*id_livro, livro)); // copia o UUID, referencia o livro
            }
        }

        if encontrados.is_empty() {
            return None;
        }

        println!("\n=== Lista de Livros ===");
        for (id, livro) in &encontrados {
            println!("ID interno: {}", id);
            println!("{}", livro);
            println!("---------------------------");
        }

        Some(encontrados)
    }
}

pub fn ler_string(mensagem: String) -> String {
    let mut entrada = String::new();
    println!("{}", mensagem);

    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler entrada");

    entrada.trim().to_string()
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

pub fn ler_i32(mensagem: String) -> i32 {
    let mut entrada = String::new();

    println!("{}", mensagem);

    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");

    let numero: i32 = entrada
        .trim() // remove \n e espaços
        .parse() // tenta converter para número
        .expect("Digite um número válido!");

    numero
}

pub fn ler_uuid(mensagem: &str) -> Uuid {
    loop {
        println!("{}", mensagem);

        let mut entrada = String::new();

        match io::stdin().read_line(&mut entrada) {
            Ok(_) => {
                let entrada = entrada.trim();

                match Uuid::parse_str(entrada) {
                    Ok(uuid) => return uuid, // ✅ retorna o UUID válido
                    Err(_) => println!("❌ UUID inválido! Tente novamente."),
                }
            }
            Err(_) => {
                println!("❌ Erro ao ler a entrada. Tente novamente.");
            }
        }
    }
}
