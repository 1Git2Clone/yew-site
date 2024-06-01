use super::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    wrap_site(html! {
        <>
            <h1>{ "Blogs" }</h1>
            <div tag={"blogs_list"} class={"blogs-list"}>
                <div tag={"understanding_big_o_notation"} class={"blog-item"}>
                    <p>
                        <Link<Route> to={Route::BlogUnderstandingBigONotation}>
                            {"Understanding Big O notation"}
                        </Link<Route>>
                    </p>
                </div>
            </div>
        </>
    })
}
