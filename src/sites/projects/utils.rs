use super::*;

/// This is a utility for systemizing the site contents. The `content_html` property is in the
/// contents module of this directory.
/// In this way it's possible to put all the Function Components in this module folder and
/// re-export them to make the project more structured.
pub fn wrap_project_subsite(content_html: Html) -> Html {
    wrap_site(html! {
        <>
            <br />
            <Link<Route> to={Route::Projects}>
                { "Go back" }
            </Link<Route>>
            {content_html}
        </>
    })
}
