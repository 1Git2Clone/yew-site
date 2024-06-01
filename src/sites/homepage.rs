use super::*;

#[function_component(Homepage)]
pub fn homepage() -> Html {
    wrap_site(html! {
        <>
            <h1>{ "What's this site?" }</h1>
            <p>
                {"This is a simple site which uses Yew.rs in order to
                    display its content. Additionally, it uses the
                    yew-router crate in order to achieve the
                    navigation functionality."}

                <br />

                {"No one cares about that though... Make sure to check out the best part of this site! ("}
                <Link<Route> to={Route::GalleryBase}>
                    {"the Hu Tao gallery"}
                </Link<Route>>
                {")"}
            </p>
        </>
    })
}
