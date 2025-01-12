use bson::oid::ObjectId;

pub fn to_object_id<S: AsRef<str>>(id: S) -> Result<ObjectId, ()> {
    ObjectId::parse_str(id.as_ref()).map_err(|_| ())
}
