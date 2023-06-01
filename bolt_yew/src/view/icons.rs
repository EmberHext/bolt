use yew::{html, Html};

pub fn github_icon(height: u32, width: u32) -> Html {
    html! {
        <svg viewBox="0 0 1024 1024" fill="currentColor" height={height.to_string() + "px"} width={width.to_string() + "px"}>
          <path d="M511.6 76.3C264.3 76.2 64 276.4 64 523.5 64 718.9 189.3 885 363.8 946c23.5 5.9 19.9-10.8 19.9-22.2v-77.5c-135.7 15.9-141.2-73.9-150.3-88.9C215 726 171.5 718 184.5 703c30.9-15.9 62.4 4 98.9 57.9 26.4 39.1 77.9 32.5 104 26 5.7-23.5 17.9-44.5 34.7-60.8-140.6-25.2-199.2-111-199.2-213 0-49.5 16.3-95 48.3-131.7-20.4-60.5 1.9-112.3 4.9-120 58.1-5.2 118.5 41.6 123.2 45.3 33-8.9 70.7-13.6 112.9-13.6 42.4 0 80.2 4.9 113.5 13.9 11.3-8.6 67.3-48.8 121.3-43.9 2.9 7.7 24.7 58.3 5.5 118 32.4 36.8 48.9 82.7 48.9 132.3 0 102.2-59 188.1-200 212.9a127.5 127.5 0 0138.1 91v112.5c.8 9 0 17.9 15 17.9 177.1-59.7 304.6-227 304.6-424.1 0-247.2-200.4-447.3-447.5-447.3z" />
        </svg>
    }
}

pub fn help_icon(height: u32, width: u32) -> Html {
    html! {
        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height={height.to_string() + "px"} width={width.to_string() + "px"} xmlns="http://www.w3.org/2000/svg"><path d="M12 6a3.939 3.939 0 0 0-3.934 3.934h2C10.066 8.867 10.934 8 12 8s1.934.867 1.934 1.934c0 .598-.481 1.032-1.216 1.626a9.208 9.208 0 0 0-.691.599c-.998.997-1.027 2.056-1.027 2.174V15h2l-.001-.633c.001-.016.033-.386.441-.793.15-.15.339-.3.535-.458.779-.631 1.958-1.584 1.958-3.182A3.937 3.937 0 0 0 12 6zm-1 10h2v2h-2z"></path><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 18c-4.411 0-8-3.589-8-8s3.589-8 8-8 8 3.589 8 8-3.589 8-8 8z"></path></svg>
    }
}

pub fn http_icon(height: u32, width: u32) -> Html {
    html! {
        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 1024 1024" height={height.to_string() + "px"} width={width.to_string() + "px"} xmlns="http://www.w3.org/2000/svg"><path d="M917.7 148.8l-42.4-42.4c-1.6-1.6-3.6-2.3-5.7-2.3s-4.1.8-5.7 2.3l-76.1 76.1a199.27 199.27 0 0 0-112.1-34.3c-51.2 0-102.4 19.5-141.5 58.6L432.3 308.7a8.03 8.03 0 0 0 0 11.3L704 591.7c1.6 1.6 3.6 2.3 5.7 2.3 2 0 4.1-.8 5.7-2.3l101.9-101.9c68.9-69 77-175.7 24.3-253.5l76.1-76.1c3.1-3.2 3.1-8.3 0-11.4zM578.9 546.7a8.03 8.03 0 0 0-11.3 0L501 613.3 410.7 523l66.7-66.7c3.1-3.1 3.1-8.2 0-11.3L441 408.6a8.03 8.03 0 0 0-11.3 0L363 475.3l-43-43a7.85 7.85 0 0 0-5.7-2.3c-2 0-4.1.8-5.7 2.3L206.8 534.2c-68.9 68.9-77 175.7-24.3 253.5l-76.1 76.1a8.03 8.03 0 0 0 0 11.3l42.4 42.4c1.6 1.6 3.6 2.3 5.7 2.3s4.1-.8 5.7-2.3l76.1-76.1c33.7 22.9 72.9 34.3 112.1 34.3 51.2 0 102.4-19.5 141.5-58.6l101.9-101.9c3.1-3.1 3.1-8.2 0-11.3l-43-43 66.7-66.7c3.1-3.1 3.1-8.2 0-11.3l-36.6-36.2z"></path></svg>
    }
}

pub fn copy_icon(height: u32, width: u32) -> Html {
    html! {
    <svg fill="none" viewBox="0 0 15 15" height={height.to_string() + "px"} width={width.to_string() + "px"}>
      <path
        fill="currentColor"
        fillRule="evenodd"
        d="M1 9.5A1.5 1.5 0 002.5 11H4v-1H2.5a.5.5 0 01-.5-.5v-7a.5.5 0 01.5-.5h7a.5.5 0 01.5.5V4H5.5A1.5 1.5 0 004 5.5v7A1.5 1.5 0 005.5 14h7a1.5 1.5 0 001.5-1.5v-7A1.5 1.5 0 0012.5 4H11V2.5A1.5 1.5 0 009.5 1h-7A1.5 1.5 0 001 2.5v7zm4-4a.5.5 0 01.5-.5h7a.5.5 0 01.5.5v7a.5.5 0 01-.5.5h-7a.5.5 0 01-.5-.5v-7z"
        clipRule="evenodd"
      />
    </svg>
    }
}

pub fn websocket_icon(height: u32, width: u32) -> Html {
    html! {
        <svg viewBox="0 0 32 32" fill="currentColor" height={height.to_string() + "px"} width={width.to_string() + "px"}>
          <path fill="currentColor" d="M21.68 20.569h2.801v-6.726l-3.156-3.156-1.981 1.981 2.335 2.336v5.565zm2.808 1.404h-9.771l-2.335-2.336.99-.99 1.929 1.929h3.969l-3.91-3.917.998-.998 3.91 3.91v-3.969l-1.922-1.922.983-.983-4.856-4.878H4.717l2.794 2.794v.007h5.795l2.047 2.047-2.993 2.993-2.047-2.047v-1.589H7.512v2.749l4.848 4.848-1.973 1.973 3.156 3.156h13.74l-2.794-2.779z" />
        </svg>
    }
}

pub fn tcp_icon(height: u32, width: u32) -> Html {
    html! {
        <svg viewBox="0 0 24 24" fill="currentColor" height={height.to_string() + "px"} width={width.to_string() + "px"} >
          <path d="M12 9c-1.3 0-2.4.8-2.8 2H6.8C6.4 9.8 5.3 9 4 9c-1.7 0-3 1.3-3 3s1.3 3 3 3c1.3 0 2.4-.8 2.8-2h2.4c.4 1.2 1.5 2 2.8 2s2.4-.8 2.8-2h2.4c.4 1.2 1.5 2 2.8 2 1.7 0 3-1.3 3-3s-1.3-3-3-3c-1.3 0-2.4.8-2.8 2h-2.4c-.4-1.2-1.5-2-2.8-2m-9 3c0-.6.4-1 1-1s1 .4 1 1-.4 1-1 1-1-.4-1-1m18 0c0 .6-.4 1-1 1s-1-.4-1-1 .4-1 1-1 1 .4 1 1z" />
        </svg>
    }
}

// pub fn udp_icon(height: u32, width: u32) -> Html {
//     html! {
//         <svg viewBox="0 0 24 24" fill="currentColor" height={height.to_string() + "px"} width={width.to_string() + "px"}>
//             <path d="M15 12c0-1.3-.84-2.4-2-2.82V6.82C14.16 6.4 15 5.3 15 4a3 3 0 00-3-3 3 3 0 00-3 3c0 1.3.84 2.4 2 2.82v2.37C9.84 9.6 9 10.7 9 12s.84 2.4 2 2.82v2.36C9.84 17.6 9 18.7 9 20a3 3 0 003 3 3 3 0 003-3c0-1.3-.84-2.4-2-2.82v-2.36c1.16-.42 2-1.52 2-2.82m-3-9a1 1 0 011 1 1 1 0 01-1 1 1 1 0 01-1-1 1 1 0 011-1m0 18a1 1 0 01-1-1 1 1 0 011-1 1 1 0 011 1 1 1 0 01-1 1z" />
//         </svg>
//     }
// }

// pub fn servers_icon(height: u32, width: u32) -> Html {
//     html! {
//         <svg fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} viewBox="0 0 24 24" height={height.to_string() + "px"} width={width.to_string() + "px"}>
//           <path stroke="none" d="M0 0h24v24H0z" />
//           <path d="M6 4 H18 A3 3 0 0 1 21 7 V9 A3 3 0 0 1 18 12 H6 A3 3 0 0 1 3 9 V7 A3 3 0 0 1 6 4 z" />
//           <path d="M6 12 H18 A3 3 0 0 1 21 15 V17 A3 3 0 0 1 18 20 H6 A3 3 0 0 1 3 17 V15 A3 3 0 0 1 6 12 z" />
//           <path d="M7 8v.01M7 16v.01" />
//         </svg>
//     }
// }

// pub fn collections_icon(height: u32, width: u32) -> Html {
//     html! {
//         <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 16 16" height={height.to_string() + "px"} width={width.to_string() + "px"} xmlns="http://www.w3.org/2000/svg"><path d="M0 13a1.5 1.5 0 0 0 1.5 1.5h13A1.5 1.5 0 0 0 16 13V6a1.5 1.5 0 0 0-1.5-1.5h-13A1.5 1.5 0 0 0 0 6v7zM2 3a.5.5 0 0 0 .5.5h11a.5.5 0 0 0 0-1h-11A.5.5 0 0 0 2 3zm2-2a.5.5 0 0 0 .5.5h7a.5.5 0 0 0 0-1h-7A.5.5 0 0 0 4 1z"></path></svg>
//     }
// }
