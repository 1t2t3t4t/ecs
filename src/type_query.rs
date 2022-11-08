use crate::entity::Entity;
use std::any::TypeId;

pub trait TypesQueryable<'e> {
    type QueryResult;
    type QueryResultMut;

    fn get_types() -> Vec<TypeId>;

    fn query(entity: &'e Entity) -> Option<Self::QueryResult>;
    fn query_mut(entity: &'e mut Entity) -> Option<Self::QueryResultMut>;
}

macro_rules! types_queryable_tuple {
    ($n: expr, $($a:tt),+; $($x:tt),+) => {
        impl<'e, $($a), +> TypesQueryable<'e> for ($($a), +) where $($a : std::any::Any),+ {
            type QueryResult = ($(&'e $a),+);
            type QueryResultMut = ($(&'e mut $a),+);

            fn get_types() -> Vec<std::any::TypeId> {
                let mut types = vec![
                    $(std::any::TypeId::of::<$a>()),+
                ];
                types.sort();
                types
            }

            fn query(entity: &'e Entity) -> Option<Self::QueryResult> {
                if !entity.has_components::<Self>() {
                    None
                } else {
                    Some((
                        $(entity.get_component::<$a>().unwrap()),+
                    ))
                }
            }

            fn query_mut(entity: &'e mut Entity) -> Option<Self::QueryResultMut> {
                let raw_components = entity.get_raw_components_mut([
                    $(&std::any::TypeId::of::<$a>()),+
                ])?;
                let [$($x),+] = raw_components;
                Some((
                    $($x.downcast_mut::<$a>()?),+
                ))
            }
        }
    }
}

// Auto implement tuple query typeid getter
types_queryable_tuple!(5, A, B, C, D, E; a, b, c, d, e);
types_queryable_tuple!(4, A, B, C, D; a, b, c, d);
types_queryable_tuple!(3, A, B, C; a, b, c);
types_queryable_tuple!(2, A, B; a, b);
