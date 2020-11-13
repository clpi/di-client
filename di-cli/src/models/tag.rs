use super::{
    Model,
};

pub struct TagKey(String);

pub struct TagInstance<T: Model>  {
    key: TagKey,
    value: Option<String>,
    attached_to: T,

}

pub struct TagAssociation {

}
