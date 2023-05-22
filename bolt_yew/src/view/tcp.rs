use crate::BoltContext;
use crate::view;
use yew::{html, Html};

pub fn tcp_view(bctx: &mut BoltContext) -> Html {
    html! {
       <body>
            {view::navbar::get_navbar(bctx)}

            <div class="main">
                <div class="sidebars">
                    {view::sidebar1::sidebar(bctx, bctx.main_state.page)}
                    {view::sidebar2::sidebar_tcp(bctx)}
                </div>

                <div class="resizer"></div>
        
                <div class="content">
                    {view::request::tcp_out(bctx)}
                                
                    <div class="resizer2"></div>     
        
                    {view::response::tcp_history(bctx)}
                </div>
            </div>

            // {view::console::console()}
        </body>
    }
}
