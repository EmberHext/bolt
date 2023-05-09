use crate::BoltContext;
use crate::view;
use yew::{html, Html};

pub fn websockets_view(bctx: &mut BoltContext) -> Html {
    html! {
       <body>
            {view::navbar::get_navbar(bctx)}

            <div class="main">
                <div class="sidebars">
                    {view::sidebar1::sidebar(bctx, bctx.main_state.page)}
                    {view::sidebar2::sidebar_websockets(bctx)}
                </div>

                <div class="resizer"></div>
        
                <div class="content">
                    {view::request::ws_out(bctx)}
                                
                    <div class="resizer2"></div>     
        
                    {view::response::ws_in(bctx)}
                </div>
            </div>

            // {view::console::console()}
        </body>
    }
}
