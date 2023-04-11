use rocket::form::FromForm;
use rocket::{get, post, serde::json::Json, routes, Rocket, Build};

#[cfg(feature = "simulator")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "simulator")]
use rocket_okapi::okapi::schemars::JsonSchema;
#[cfg(feature = "simulator")]
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
//use rocket_okapi::settings::UrlObject;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "simulator")]
    {
        pub mod simulator;
        use simulator::*;
    }
    else
    {
        pub mod target;
        use target::*;
    }
}

// #[cfg(feature = "simulator")]
// pub mod simulator;
// #[cfg(feature = "simulator")]
// use simulator::*;

// #[cfg(not(feature = "simulator"))]
// pub mod target; 
// #[cfg(not(feature = "simulator"))]
// use target::*;


#[cfg_attr(feature = "simulator", derive(JsonSchema))]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: u64,
    username: String,
    #[cfg_attr(feature = "simulator", schemars(example = "example_email"))]
    email: Option<String>,
}

fn example_email() -> &'static str {
    "test@example.com"
}

/// # Get all users
///
/// Returns all users in the system.
#[cfg_attr(feature = "simulator", openapi(tag = "Users"))]
#[get("/user")]
fn get_all_users() -> Json<Vec<User>> {
    get_users();
    Json(vec![User {
        user_id: 42,
        username: "bob".to_owned(),
        email: None,
    }])
}

/// # Get user
///
/// Returns a single user by ID.
#[cfg_attr(feature = "simulator", openapi(tag = "Users"))]
#[get("/user/<id>")]
fn get_user(id: u64) -> Option<Json<[User; 2]>> {
    Some(Json([User {
        user_id: id,
        username: "bob".to_owned(),
        email: None,
    }, User { user_id: id, username: "foobar".to_owned(), email: Some(example_email().to_owned()) }]))
}

/// # Get user by name
///
/// Returns a single user by username.
#[cfg_attr(feature = "simulator", openapi(tag = "Users"))]
#[get("/user_example?<user_id>&<name>&<email>")]
fn get_user_by_name(user_id: u64, name: String, email: Option<String>) -> Option<Json<User>> {
    Some(Json(User {
        user_id,
        username: name,
        email,
    }))
}

/// # Create user
#[cfg_attr(feature = "simulator", openapi(tag = "Users"))]
#[post("/user", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    user
}

#[cfg_attr(feature = "simulator", openapi(skip))]
#[get("/hidden")]
fn hidden() -> Json<&'static str> {
    Json("Hidden from swagger!")
}

#[cfg_attr(feature = "simulator", derive(JsonSchema))]
#[derive(Serialize, Deserialize, FromForm)]
struct Post {
    /// The unique identifier for the post.
    post_id: u64,
    /// The title of the post.
    title: String,
    /// A short summary of the post.
    summary: Option<String>,
}

/// # Create post using query params
///
/// Returns the created post.
#[cfg_attr(feature = "simulator", openapi(tag = "Posts"))]
#[get("/post_by_query?<post..>")]
fn create_post_by_query(post: Post) -> Option<Json<Post>> {
    Some(Json(post))
}

#[cfg(not(feature = "simulator"))]
fn swagger_setup(builder: Rocket<Build>) -> Rocket<Build>
{
    builder
}

#[cfg(feature = "simulator")]
fn swagger_setup(builder: Rocket<Build>) -> Rocket<Build>
{
    builder.mount(
        "/swagger/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    )
}

#[cfg(feature = "simulator")]
macro_rules! my_get_routes {
    ($($x:tt)*) => {
        openapi_get_routes![$($x)*]
    }
}

#[cfg(not(feature = "simulator"))]
macro_rules! my_get_routes {
    ($($x:tt)*) => {
        routes![$($x)*]
    }
}

#[rocket::main]
async fn main() {
    let mut launch_result = rocket::build();
    launch_result = launch_result
        .mount(
            "/",
            my_get_routes![
                get_all_users,
                get_user,
                get_user_by_name,
                create_user,
                hidden,
                create_post_by_query,
            ],
        );
    println!("Foobar !!!");
    launch_result = swagger_setup(launch_result);
    let launch_result = launch_result
        .launch()
        .await;

        // .mount(
        //     "/rapidoc/",
        //     make_rapidoc(&RapiDocConfig {
        //         general: GeneralConfig {
        //             spec_urls: vec![UrlObject::new("General", "../openapi.json")],
        //             ..Default::default()
        //         },
        //         hide_show: HideShowConfig {
        //             allow_spec_url_load: false,
        //             allow_spec_file_load: false,
        //             ..Default::default()
        //         },
        //         ..Default::default()
        //     }),
        // )
        //.launch()
        //.await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}