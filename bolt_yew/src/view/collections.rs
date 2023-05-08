use crate::BoltContext;
use crate::view;
use yew::{html, Html};

pub fn collections_view(bctx: &mut BoltContext) -> Html {
    html! {
       <body>
            {view::navbar::get_navbar(bctx)}

            <div class="main">
                <div class="sidebars">
                    {view::sidebar1::sidebar(bctx, bctx.page)}
                    {view::sidebar2::sidebar_collections(bctx)}
                </div>

                <div class="resizer"></div>
        
                <div class="content">
                    {view::request::collection_request(bctx)}
                                
                    <div class="resizer2"></div>     
        
                    {view::response::collection_response(bctx)}
                </div>
            </div>

            // {view::console::console()}
        </body>
    }
}
