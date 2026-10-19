use check::{auth::password::hash_password, entities::{courses, departments, enrollments, prelude::*, users}};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, Database, EntityTrait, QueryFilter};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set on .env");

    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Seeding database");

    // 1. Find or create a department
    let department = match Departments::find()
        .filter(departments::Column::Name.eq("Computer Science"))
        .one(&db)
        .await
        .expect("DB query failed") {
            Some(dept) => {
                println!("Department already exitst: {}", dept.name);
                dept
            }
            None => {
                let dept = departments::ActiveModel {
                    name: Set("Computer Science".to_string()),
                    ..Default::default()
                }
                .insert(&db)
                .await
                .expect("Failed to insert department");
                println!("Created department: {}", dept.name);
                dept
            }
        };


    // 2. find or create a Professor
    let professor = match Users::find()
        .filter(users::Column::Email.eq("azard@prof.com"))
        .one(&db)
        .await
        .expect("DB query failed") {
            Some(user) => {
                println!("Professor already exist: {}", user.name);
                user
            }
            None => {
                let professor_password = hash_password("professor123").expect("Hash failed");
                let user = users::ActiveModel {
                    name: Set("Azad".to_string()),
                    email: Set("azard@prof.com".to_string()),
                    role: Set("professor".to_string()),
                    password: Set(professor_password),
                    department_id: Set(Some(department.id)),
                    ..Default::default()
                }
                .insert(&db)
                .await
                .expect("Failed to insert Professor");
                println!("created professor: {}", user.name);
                user
            }
        };

    

    // 3. find or create a student
    let student = match Users::find()
        .filter(users::Column::Email.eq("hasan@stud.com"))
        .one(&db)
        .await
        .expect("DB query failed") {
            Some(user) => {
                println!("Student already exist: {}", user.name);
                user
            }
            None => {
                let student_password = hash_password("student123").expect("Hash failed");
                let user = users::ActiveModel {
                    name: Set("Hasan".to_string()),
                    email: Set("hasan@stud.com".to_string()),
                    role: Set("student".to_string()),
                    password: Set(student_password),
                    department_id: Set(Some(department.id)),
                    ..Default::default()
                }
                .insert(&db)
                .await
                .expect("Failed to insert Student");
                println!("created student: {}", user.name);
                user
            }
        };

    // 4. find or create an admin
    if Users::find()
        .filter(users::Column::Email.eq("admin@admin.com"))
        .one(&db)
        .await
        .expect("DB query failed")
        .is_none()
        {
            let admin_password = hash_password("admin123").expect("Hash failed");
            let user = users::ActiveModel {
                name: Set("Admin".to_string()),
                email: Set("admin@admin.com".to_string()),
                role: Set("admin".to_string()),
                password: Set(admin_password),
                ..Default::default()
            }
            .insert(&db)
            .await
            .expect("Failed to insert Admin");
            println!("created admin: {}", user.name);
        } else {
            println!("Admin user already exist!!");
        }

    // 5. find or create a course
    let course = match Courses::find()
        .filter(courses::Column::Title.eq("Introduction to Rust"))
        .one(&db)
        .await
        .expect("DB query failed")
        {
            Some(c) => {
                println!("Course already exist: {}", c.title);
                c
            }
            None => {
                let c = courses::ActiveModel {
                    title: Set("Introduction to Rust".to_string()),
                    fee: Set(rust_decimal::Decimal::new(50000, 2)),
                    instructor_id: Set(Some(professor.id)),
                    ..Default::default()
                }
                .insert(&db)
                .await
                .expect("Failed to insert course");
                println!("Created course: {}", c.title);
                c
            }
        };


    // 6. find or create enrollment
    if Enrollments::find()
        .filter(enrollments::Column::StudentId.eq(student.id))
        .filter(enrollments::Column::CourseId.eq(course.id))
        .one(&db)
        .await
        .expect("DB query failed")
        .is_none()
        {
            enrollments::ActiveModel{
                student_id: Set(Some(student.id)),
                course_id: Set(Some(course.id)),
                ..Default::default()
            }
            .insert(&db)
            .await
            .expect("Failed to insert enrollment");
            println!("enrolled {} in {}", student.name, course.title);
        } else {
            println!("{} is already enrolled in {}", student.id, course.id);
        }
        

    println!("\nSeeding complete!");
    println!("\nLogin credentials:");
    println!("  Admin:     admin@admin.com / admin123");
    println!("  Professor: azad@prof.com / professor123");
    println!("  Student:   hasan@stud.com / student123");
}