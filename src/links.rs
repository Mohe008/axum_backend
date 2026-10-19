use sea_orm::{Linked, RelationDef, RelationTrait};

use crate::entities::{users, courses, enrollments};

pub struct UserCourses;

impl Linked for UserCourses {
    type FromEntity = users::Entity;
    type ToEntity = courses::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            users::Relation::Enrollments.def(),
            enrollments::Relation::Courses.def(),
        ]
    }
}

pub struct CourseUsers;

impl Linked for CourseUsers {
    type FromEntity = courses::Entity;
    type ToEntity = users::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            courses::Relation::Enrollments.def(),
            enrollments::Relation::Users.def(),
        ]
    }
}