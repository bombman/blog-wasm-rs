use crate::views::home::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/style.css"/>

        // sets the document title
        <Title text="Welcome to Myblog"/>

        // content for this welcome page
        <Router>
            <main>
            <div class="bg-white font-family-karla flex justify-center">

                <Routes>
                    <Route path="" view=|cx| view! { cx, <Home/> }/>
                </Routes>
            </div>
            </main>
        </Router>
    }
}
