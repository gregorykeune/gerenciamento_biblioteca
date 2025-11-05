use uuid::Uuid;

#[derive(Debug)]
pub enum ErroBiblioteca {
    LivroNaoEncontrado(Uuid),
    UsuarioNaoEncontrado(Uuid),
    EmprestimoNaoEncontrado(Uuid),
    EstadoInvalido(String),
    ErroPersistencia(String),
}