#[cfg(feature = "ssr")]
use std::fs::File;

use leptos::{
    server_fn::codec::{GetUrl, Json},
    *,
};
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::components::{
    about::About, certifications::Certifications, contact::Contact, experience::Experience,
    hero::Hero, navbar::NavBar, skill::Skill,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub title: String,
    pub duration: String,
    pub responsibilities: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Experience {
    pub company: String,
    pub company_url: String,
    pub logo: String,
    pub location: String,
    pub positions: Vec<Position>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    pub title: String,
    pub category: String,
    pub proficiency: String,
    pub proficiency_level: u8,
    pub color: String,
    pub icon: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Certification {
    pub title: String,
    pub provider: String,
    pub date: String,
    pub link: String,
    pub badge: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contact {
    pub contact_type: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub name: String,
    pub title: String,
    pub about: String,
    pub skills: Vec<Skill>,
    pub experience: Vec<Experience>,
    pub certifications: Vec<Certification>,
    pub contacts: Vec<Contact>,
}

/// Loads data from a JSON file
#[server(
    name = LoadData,
    prefix = "/api",
    endpoint = "load-data",
    input = GetUrl,
    output = Json
)]
pub async fn load_data() -> Result<Data, ServerFnError> {
    // TODO: Load from JSON
    logging::log!("loading data...");
    let file = File::open("./data/data.json").unwrap();
    let data: Data = serde_json::from_reader(file).unwrap();
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

        // content for this welcome page
        <Router>
            <NavBar/>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (data_loaded, set_data_loaded) = create_signal(false);
    let (data, set_data) = create_signal(Data {
        name: "".to_string(),
        title: "".to_string(),
        about: "AAA".to_string(),
        skills: vec![],
        experience: vec![],
        certifications: vec![],
        contacts: vec![],
    });

    let (name, set_name) = create_signal(data().name);
    let (title, set_title) = create_signal(data().title);
    let (about, set_about) = create_signal(data().about.clone());
    let (exp, set_exp) = create_signal(data().experience.clone());
    let (skills, set_skills) = create_signal(data().skills.clone());
    let (certs, set_certs) = create_signal(data().certifications.clone());
    let (contacts, set_contacts) = create_signal(data().contacts.clone());

    // let once = create_resource(
    //     || (),
    //     |_| async move {
    //         logging::log!("Resource loading");
    //         let data_result = load_data().await;
    //         match data_result {
    //             Ok(data) => Some(data),
    //             Err(e) => None,
    //         }
    //     },
    // );

    create_effect(move |_| {
        logging::log!("Data loaded: {}", data_loaded());

        spawn_local(async move {
            logging::log!("Loading data...");
            // only load the data once on initial render
            if !data_loaded() {
                let data_result = load_data().await;
                if let Ok(data) = data_result {
                    set_data.update(|user_data: &mut Data| *user_data = data.clone());
                    set_data_loaded.update(|data_loaded: &mut bool| *data_loaded = true);
                    set_name.update(|name: &mut String| *name = data.name.clone());
                    set_title.update(|title: &mut String| *title = data.title.clone());
                    set_about.update(|about: &mut String| *about = data.about.clone());
                    set_exp.update(|exp: &mut Vec<Experience>| *exp = data.experience.clone());
                    set_skills.update(|skills: &mut Vec<Skill>| *skills = data.skills.clone());
                    set_certs.update(|certs: &mut Vec<Certification>| {
                        *certs = data.certifications.clone()
                    });
                    set_contacts
                        .update(|contacts: &mut Vec<Contact>| *contacts = data.contacts.clone());
                }
            }
        })
    });

    view! {
        <main>
            <Hero name=name title=title />
            <About about_me_text=about/>
            <Experience experiences=exp/>
            <Skill skills=skills/>
            <Certifications certs=certs/>
            <Contact contacts=contacts/>
        </main>
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
        <h1>"Not Found"</h1>
    }
}
