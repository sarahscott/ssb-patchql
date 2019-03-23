use super::post::*;
use crate::db::schema::messages::dsl::{
    content_type as messages_content_type, key_id as messages_key_id, messages as messages_table,
    root_key_id as messages_root_key_id,
};
use crate::db::Context;
use diesel::prelude::*;

#[derive(Default)]
pub struct Thread {
    pub root: Post,
}

graphql_object!(Thread: Context |&self| {
    field root(&executor) -> &Post {
        &self.root
    }
    field replies(&executor) -> Vec<Post>{
        let connection = executor.context().connection.lock().unwrap();

        messages_table
            .select(messages_key_id)
            .filter(messages_root_key_id.eq(self.root.key_id))
            .filter(messages_content_type.eq("post"))
            .load::<i32>(&(*connection))
            .into_iter()
            .flatten()
            .map(|key_id|{
                Post{key_id}
            })
            .collect::<Vec<Post>>()
    }
    field is_private() -> bool {false}
});
