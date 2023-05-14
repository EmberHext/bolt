use crate::BoltContext;
use crate::view;
use yew::{html, Html};

pub fn http_view(bctx: &mut BoltContext) -> Html {
    html! {
       <body>
            {view::navbar::get_navbar(bctx)}

            <div class="main">
                <div class="sidebars">
                    {view::sidebar1::sidebar(bctx, bctx.main_state.page)}
                    {view::sidebar2::sidebar_http(bctx)}
                </div>
                
                <div class="resizer"></div>
        
                <div class="content">
                    {view::request::http_request(bctx)}

                    <div class="resizer2"></div>
        
                    {view::response::http_response(bctx)}
                </div>
            </div>

       </body>
    }
}
