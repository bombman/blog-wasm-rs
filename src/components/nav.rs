use leptos::*;
#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
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
                                </div>
                            </div>
                        </div>
            </nav>
        </div>
    }
}
