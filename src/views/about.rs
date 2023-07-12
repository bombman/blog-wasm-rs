use leptos::*;
#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! { cx,

        <div class="flex flex-col items-center py-12">
            <a class="font-bold text-gray-800 uppercase hover:text-gray-700 text-5xl" href="#">
               {"About"}
            </a>

        </div>

    }
}
