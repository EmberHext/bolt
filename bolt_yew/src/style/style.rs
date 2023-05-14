use stylist::css;

pub fn _get_styles() -> stylist::StyleSource {
    css!(
        r#"
* {
	margin: 0;
}

body {
	cursor: default;
	user-select: none;
	color: white;
	font-family: Arial, Helvetica, sans-serif;
}

"#
    )
}
