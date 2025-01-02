pub async fn create_task(&self, task: &TodoItem) -> Result<(), diesel::result::Error> {
    let conn = &mut self.db_pool.get().unwrap();

    let _ = diesel::insert_into(crate::schema::tasks::table)
        .values(task)
        .execute(conn)?;

    Ok(())
}

pub async fn list_tasks(&self) -> Result<Vec<TodoList>, diesel::result::Error> {
    let conn = &mut self.db_pool.get().unwrap();

    let tasks = crate::schema::tasks::table.load::<TodoList>(conn)?;

    Ok(tasks)
}

pub async fn list_task(&self, id: i32) -> Result<Vec<TodoList>, diesel::result::Error> {
    let conn = &mut self.db_pool.get().unwrap();

    let tasks = crate::schema::tasks::table
        .filter(crate::schema::tasks::task_id.eq(id))
        .load::<TodoList>(conn)?;

    Ok(tasks)
}

pub async fn update_task(&self, id: i32) -> Result<(), diesel::result::Error> {
    let conn = &mut self.db_pool.get().unwrap();

    let _ = diesel::update(crate::schema::tasks::table)
        .filter(crate::schema::tasks::task_id.eq(id))
        .set(crate::schema::tasks::complete.eq(true))
        .execute(conn)?;

    Ok(())
}

pub async fn delete_task(&self, id: i32) -> Result<(), diesel::result::Error> {
    let conn = &mut self.db_pool.get().unwrap();

    let _ = diesel::delete(crate::schema::tasks::table)
        .filter(crate::schema::tasks::task_id.eq(id))
        .execute(conn)?;

    Ok(())
}
