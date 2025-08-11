use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool, PoolConnection, PoolError};
use std::error;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooleedConnection<ConnectionManager<PgConnection>>;

#[derive(Debug)]
pub struct PgClient {
    pool: DbPool,
}

impl PgClient {
    fn new() -> Result<Self, Box<dyn error::Error>> {}
}

#[cfg(test)]
mod tests {}
