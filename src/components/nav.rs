use leptos::*;
use leptos_router::*;
#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    let location = use_location(cx);

    let (bt_home, set_bt_home) = create_signal(cx, "border-b-4".to_string());
    let (bt_about, set_bt_about) = create_signal(cx, "".to_string());

    let menu_effect = move |_| {
        let style_link = "border-b-4".to_string();
        leptos::log!("menu_effect");
        leptos::log!("{}", bt_home.get());
        let menu_name = location.pathname.get().to_lowercase();
        leptos::log!("{}", menu_name);

        set_bt_home("".to_string());
        set_bt_about("".to_string());

        if menu_name == "/" {
            set_bt_home(style_link);
        } else if menu_name == "/about" {
            set_bt_about(style_link);
        }
    };

    view! {cx,
        <div>
            <nav class="bg-white shadow-lg ">
                        <div class="max-w-6xl mx-auto px-4">
                            <div class="flex justify-between">
                                <div class="flex space-x-7">
                                    <div>
                                        <a href="#" class="flex items-center py-4 px-2">
                                            <span class="font-semibold text-blue-500 text-lg">{"Blog"}</span>
                                        </a>
                                     </div>

                                     <div class="hidden md:flex items-center space-x-1">
                                        <a href="/" on:click=menu_effect class=move || format!("py-4 px-2 text-pink-500  border-blue-300 font-semibold {}",{bt_home.get()}) >{"Home"}</a>
                                     </div>
                                     <div class="hidden md:flex items-center space-x-1">
                                        <a href="about" on:click=menu_effect class=move || format!("py-4 px-2 text-pink-500  border-blue-300 font-semibold {}",{bt_about.get()}) >{"About"}</a>
                                  </div>


                                </div>
                            </div>
                        </div>
            </nav>
        </div>
    }
}
