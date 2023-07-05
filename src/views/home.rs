use leptos::*;
#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,

        <div class="flex flex-col items-center py-12">
            <a class="font-bold text-gray-800 uppercase hover:text-gray-700 text-5xl" href="#">
               {"Welcome to MyBlog"}
            </a>
            <p class="text-lg text-gray-600">
                {"hello world"}
            </p>
        </div>

    }
}
