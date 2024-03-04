#[cfg(feature = "ssr")]
use std::fs::File;

use leptos::{
    server_fn::codec::{GetUrl, Json},
    *,
};

// use icondata as i;
// use leptos_icons::*;

use leptos_meta::*;
use leptos_router::*;

use crate::{
    components::{
        about::About, certifications::Certifications, contact::Contact, experience::Experience,
        hero::Hero, navbar::NavBar, skill::Skill,
    },
    model::user::{Certification, Contact, Experience, Skill, User},
};

/// Loads data from a JSON file
#[server(
    name = LoadData,
    prefix = "/api",
    endpoint = "load-data",
    input = GetUrl,
    output = Json
)]
pub async fn load_data() -> Result<User, ServerFnError> {
    // TODO: Load from JSON
    logging::log!("loading data...");
    // print current working directory

    let file = match File::open("./data/data.json") {
        Ok(file) => file,
        Err(error) => {
            logging::log!("Error opening file: {}", error);
            return Err(ServerFnError::ServerError(error.to_string()));
        }
    };
    let data: User = serde_json::from_reader(file).unwrap();
    Ok(data)
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>

        // sets the document title
        <Title text="Vasanth's Portfolio"/>

        <Meta name="description" content="Software Engineer Portfolio website build with Rust using Leptos Framework and Tailwind CSS"/>
        <Meta name="keywords" content="software engineer, programming, python, go, javascript, web development, devops, linux, computer science, rust, leptos, portfolio"/>
        <Meta name="author" content="Vasantha Kumar"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta property="og:title" content="Vasantha Kumar - Software Engineer"/>
        <Meta property="og:description" content="Software Engineer Portfolio website build with Rust using Leptos Framework and Tailwind CSS"/>
        <Meta property="og:image" content="https://www.vasanthakumar.dev/assets/images/profile.webp"/>
        <Meta property="og:url" content="https://www.vasanthakumar.dev"/>
        <Meta property="og:type" content="website"/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content="Vasantha Kumar - Software Engineer"/>
        <Meta name="twitter:description" content="Software Engineer Portfolio website build with Rust using Leptos Framework and Tailwind CSS"/>
        <Meta name="twitter:image" content="https://www.vasanthakumar.dev/assets/images/profile.webp"/>


        // content for this welcome page
        <Router>
            <NavBar/>
            <main>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
            // <footer class="fixed p-4 bg-gradient-to-r from-gray-800 via-gray-900 to-black flex flex-col justify-center items-center">
            //    <Icon icon={i::AiHeartFilled} class="text-red-500 text-center hover:scale-150 animate-pulse cursor-pointer" />
            // </footer>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn Home() -> impl IntoView {
    let (data_loaded, set_data_loaded) = create_signal(false);
    let (data, set_data) = create_signal(User::new());
    let (name, set_name) = create_signal(data().name);
    let (title, set_title) = create_signal(data().title);
    let (about, set_about) = create_signal(data().about.clone());
    let (exp, set_exp) = create_signal(data().experience.clone());
    let (skills, set_skills) = create_signal(data().skills.clone());
    let (certs, set_certs) = create_signal(data().certifications.clone());
    let (contacts, set_contacts) = create_signal(data().contacts.clone());

    create_local_resource(
        || (),
        move |_| async move {
            logging::log!("Resource loading");
            let data_result = load_data().await;
            if let Ok(data) = data_result {
                logging::log!("Data loaded {}", data.about.clone());
                set_data.update(|user_data: &mut User| *user_data = data.clone());
                set_data_loaded.update(|data_loaded: &mut bool| *data_loaded = true);
                set_name.update(|name: &mut String| *name = data.name.clone());
                set_title.update(|title: &mut String| *title = data.title.clone());
                set_about.update(|about: &mut String| *about = data.about.clone());
                set_exp.update(|exp: &mut Vec<Experience>| *exp = data.experience.clone());
                set_skills.update(|skills: &mut Vec<Skill>| *skills = data.skills.clone());
                set_certs
                    .update(|certs: &mut Vec<Certification>| *certs = data.certifications.clone());
                set_contacts
                    .update(|contacts: &mut Vec<Contact>| *contacts = data.contacts.clone());
            } else {
                set_data_loaded.update(|data_loaded: &mut bool| *data_loaded = false);
                logging::log!("Error loading data");
            }
        },
    );

    create_effect(move |_| {
        logging::log!("Data loaded: {}", data_loaded());
    });

    view! {
        <Hero name=name title=title/>
        <About about_me_text=about/>
        <Experience experiences=exp/>
        <Skill skills=skills/>
        <Certifications certs=certs/>
        <Contact contacts=contacts/>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <main class="w-full h-screen flex flex-col justify-center items-center text-white text-4xl bg-gradient-to-r from-gray-800 via-gray-900 to-black">
            <p class="font-bold font-montserrat uppercase ">{"404 - Not Found"}</p>
        </main>
    }
}
