mod biblioteca;
use uuid::Uuid;

pub trait Identificavel {
    fn id(&self) -> Uuid;
}

// pub trait Identificavel {
//     fn id(&self) -> Uuid;

//     fn buscar_item_pelo_id<'a, T>(colecao: &'a HashMap<Uuid, T>, id: &Uuid) -> Option<&'a T>
//     where
//         T: Identificavel,
//     {
//         colecao.get(id)
//     }
// }
