use stylist::css;
use stylist::StyleSource;

pub struct Theme {
    pub background_color: String,
    pub foreground_color: String,
    pub accent_color: String,
    pub font_color: String,
    pub default_border: i32,
    pub css: StyleSource,
}

// LightTheme uses eggshell (#f5f5f5), baby_blue (#b8c4f5), and purple (#9f66ad)
pub fn light_theme() -> Theme {
    let background_color = String::from("#c5c5d5");
    let foreground_color = String::from("#b8c4f5");
    let accent_color = String::from("#9f66ad");
    let light_accent_color = String::from("#b8c4f5");
    let font_color = String::from("#333");
    let default_border = 1;

    let css = css!(r#"
        html, body {
            font-family: sans-serif;
            padding: 1rem;
            margin: 1rem;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            flex-direction: column;
            background-color: ${bg};
            color: ${ft_color};    
        }

        /* button styles */
        button {
            background-color: ${bg};
            color: ${ft_color};
            border: ${default_border}px solid ${accent_color};
            border-radius: 5px;
            padding: 5px;
            margin: 5px;
            cursor: pointer;
        }

        button:hover {
            background-color: ${accent_color};
            color: ${bg};
        }

        /* input styles */
        input {
            background-color: ${bg};
            color: ${ft_color};
            border: ${default_border}px solid ${accent_color};
            border-radius: 5px;
            padding: 5px;
            margin: 5px;
        }

        input:hover {
            background-color: ${light_accent_color};
            color: ${ft_color};
        }
    "#,
    bg = background_color, ft_color = font_color, default_border = default_border, accent_color = accent_color, light_accent_color = light_accent_color);
    
    Theme {
        background_color,
        foreground_color,
        accent_color,
        font_color,
        default_border,
        css,
    }
}

// light components
pub fn foreground() -> StyleSource {
    let color = String::from("#e5e5e5");
    let padding = String::from("10px");
    css!(
        r#"background-color: ${color};
        padding: ${padding};"#,
        color = color,
        padding = padding
    )
}
