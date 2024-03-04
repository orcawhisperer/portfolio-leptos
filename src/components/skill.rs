use crate::model::user::Skill;
use icondata as i;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn Skill(skills: ReadSignal<Vec<Skill>>) -> impl IntoView {
    // update about me text with live typing effect (debounced)

    let (active_category, set_active_category) = create_signal("Programming");
    let (hovered_skill_index, set_hovered_skill_index) = create_signal(128);

    let ref categories = ["Programming", "DevOps & Cloud", "OS & Databases"];

    create_effect(move |_| {
        // logging::log!("skills = {:?}", skills.get().clone());
        logging::log!("active_category = {}", active_category.get());
        logging::log!("hovered_skill_index = {}", hovered_skill_index.get());
    });

    fn render_icon(icon: &str) -> View {
        match icon {
            "Python" => view! {
                <Icon class="text-[#3776AB]" icon=i::BiPython width="25" height="25"  />
            },
            "Go" => view! {
                <Icon class="text-[#00ADD8]" icon=i::BiGoLang width="25" height="25"  />
            },
            "Rust" => view! {
                <Icon class="text-[#DEA584]" icon=i::SiRust width="25" height="25"  />
            },
            "C/C++" => view! {
                <Icon class="text-yellow-500" icon=i::BiCPlusPlus width="25" height="25"  />
            },
            "JavaScript" => view! {
                <Icon class="text-[#F7DF1E]" icon=i::BiJavascript width="25" height="25"  />
            },
            "Java" => view! {
                <Icon class="text-[#E32636]" icon=i::BiJava width="25" height="25"  />
            },
            "gRPC" => view! {
                <Icon class="text-[#6CE05C]" icon=i::BiServerRegular width="25" height="25"  />
            },
            "AWS" => view! {
                <Icon class="text-[#FF9900]" icon=i::BiAws width="25" height="25"  />
            },
            "GCP" => view! {
                <Icon class="text-[#4285F4]" icon=i::BiGoogleCloud width="25" height="25"  />
            },
            "Azure" => view! {
                <Icon class="text-[#0078D7]" icon=i::BiMicrosoft width="25" height="25"  />
            },
            "Docker" => view! {
                <Icon class="text-[#2496ED]" icon=i::BiDocker width="25" height="25"  />
            },
            "Kubernetes" => view! {
                <Icon class="text-[#326CE5]" icon=i::BiKubernetes width="25" height="25"  />
            },
            "Terraform" => view! {
                <Icon class="text-[#5C4EE5]" icon=i::SiTerraform width="25" height="25"  />
            },
            "Jenkins" => view! {
                <Icon class="text-[#D24939]" icon=i::SiJenkins width="25" height="25"  />
            },
            "ShellScript" => view! {
                <Icon class="text-[#89E051]" icon=i::BsTerminal width="25" height="25"  />
            },
            "Linux" => view! {
                <Icon class="text-[#FFFFFF]" icon=i::SiLinux width="25" height="25"  />
            },
            "MacOS" => view! {
                <Icon class="text-[#A1A1A1]" icon=i::SiApple width="25" height="25"  />
            },
            "Windows" => view! {
                <Icon class="text-[#0078D7]" icon=i::SiWindows width="25" height="25"  />
            },
            "Neo4j" => view! {
                <Icon class="text-[#00A98F]" icon=i::SiNeo4j width="25" height="25"  />
            },
            "MySQL" => view! {
                <Icon class="text-[#4479A1]" icon=i::SiMysql width="25" height="25"  />
            },
            "PostgreSQL" => view! {
                <Icon class="text-[#336791]" icon=i::SiPostgresql width="25" height="25"  />
            },
            _ => view! {
                <Icon class="text-[#89E051]" icon=i::BsTerminal width="25" height="25"  />
            },
        }
        .into_view()
    }

    view! {
        <section
        id="skills"
        class="py-16 bg-gradient-to-r from-gray-800 via-gray-900 to-black">
        <h2 class="text-4xl font-bold mb-8 text-center text-white shadow-text font-montserrat">
           SKILLS
        </h2>
        <div class="container mx-auto">
           <div class="flex justify-center mb-8 space-x-4 overflow-x-auto flex-wrap">

             {
                categories.iter().enumerate().map(|(index, category)| view!{
                    <button key={index}
                        class="text-white font-semibold py-2 px-4 rounded font-montserrat mb-4"
                        class=("bg-gray-600", move || active_category.get() != *category)
                        class=("bg-gradient-to-br", move || active_category.get() == *category)
                        class=("from-blue-600", move || active_category.get() == *category)
                        class=("to-purple-500", move || active_category.get() == *category)
                        on:click=move |_| set_active_category.update(|active_category: &mut &str| *active_category = category) >
                        {category.to_string()}
                    </button>
                }).collect_view()
             }
           </div>
           <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-8">
            {
               move || skills().clone().iter().enumerate().map(|(idx, skill)| {

                    if skill.category == *active_category.get() {
                       Some( view! {
                        <div key={idx} class="flex flex-col items-center">
                            <div class={format!("text-4xl mb-4 {} cursor-pointer transform hover:scale-150 transition-all duration-300 ease-in-out", skill.clone().color)}>
                                <a href={skill.clone().url} target="_blank">
                                   {render_icon(&skill.title.clone().to_string())}
                                </a>
                            </div>
                            <h4 class="text-xl font-semibold mb-2 text-gray-300 font-montserrat">
                                        {skill.clone().title}
                            </h4>
                            <div
                                class="w-full h-6 relative overflow-hidden rounded-xl bg-gray-700"
                                on:mouseenter=move |_| {
                                    if hovered_skill_index.get() != idx as usize {
                                        set_hovered_skill_index.update(|hovered_skill_index: &mut usize| *hovered_skill_index = idx as usize)
                                    }
                                }
                                on:mouseleave=move |_| {
                                    if hovered_skill_index.get() != 128 {
                                        set_hovered_skill_index.update(|hovered_skill_index: &mut usize| *hovered_skill_index = 128)
                                    }
                                }>
                            <div
                                class="w-full h-full absolute left-0 top-0 bg-gradient-to-r from-blue-600 via-blue-500 to-purple-600 skill-bar"
                                class=("slide-in", move || hovered_skill_index.get() == idx)
                                style={format!("clip-path: polygon(0% 0%, 95% 0%, 100% 100%, 0% 100%); transform-origin: left; width: {}%; --skill-width: {}% ", skill.proficiency_level.clone(), skill.proficiency_level.clone())}
                            />

                            {
                                if hovered_skill_index.get() == idx {
                                    Some(view! {
                                        <div class="absolute inset-0 flex items-center justify-center">
                                            <span class="text-white text-sm font-semibold font-montserrat">
                                               {skill.clone().proficiency_level}%
                                            </span>
                                         </div>
                                    })
                                } else {
                                    None
                                }
                            }

                            </div>

                        </div>
                    }.into_any())
                    } else {
                        None
                    }
                }).collect_view()
            }

           </div>
        </div>
     </section>
    }
}
