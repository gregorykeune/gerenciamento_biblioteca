mod biblioteca;
use std::collections::HashMap;
use uuid::Uuid;


pub trait Identificavel {
    fn id(&self) -> Uuid;

    fn buscar_pelo_id<'a, T>(colecao: &'a HashMap<Uuid, T>, id: &Uuid) -> Option<&'a T>
    where
        T: Identificavel,
    {
        colecao.get(id)
    }
}
