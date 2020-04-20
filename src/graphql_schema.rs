extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use juniper::RootNode;
use std::env;

use crate::objects::member::{Member, NewMember};
use crate::objects::team::Team;

pub struct QueryRoot;
pub struct MutationRoot;

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connectiong to {}", database_url))
}

#[juniper::object]
impl QueryRoot {
    fn members() -> Vec<Member> {
        use crate::schema::members::dsl::*;
        let connection = establish_connection();
        members
            .limit(100)
            .load::<Member>(&connection)
            .expect("Error loading members")
    }

    fn teams() -> Vec<Team> {
        use crate::schema::teams::dsl::*;
        let connection = establish_connection();
        teams
            .limit(10)
            .load::<Team>(&connection)
            .expect("Error loading teams")
    }
}

#[juniper::object]
impl MutationRoot {
    fn create_member(data: NewMember) -> Member {
        use crate::schema::members as dbmembers;
        let connection = establish_connection();
        diesel::insert_into(dbmembers::table)
            .values(&data)
            .get_result(&connection)
            .expect("Error saving new post")
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
