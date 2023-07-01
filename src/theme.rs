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
            margin: 0;
            padding: 0;
            height: 100%;
            width: 100%;
        }

        .page {
            font-family: sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100%;
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

        /* dropdown styles */
        .dropdown {
            position: relative;
            display: inline-block;
        }

        .dropdown-content {
            display: none;
            position: absolute;
            background-color: ${bg};
            min-width: 160px;
            box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
            border: ${default_border}px solid ${accent_color};
            z-index: 1;
        }

        .dropdown-content a {
            color: ${ft_color};
            padding: 12px 16px;
            text-decoration: none;
            display: block;
        }

        .dropdown-content a:hover {
            background-color: ${accent_color};
            color: ${bg};
        }

        .dropdown:hover .dropdown-content {
            display: block;
        }

        .dropdown:hover .dropbtn {
            background-color: ${accent_color};
            color: ${bg};
        }

        /* navbar styles */

        .navbar {
            overflow: hidden;
            background-color: ${bg};
            position: fixed;
            top: 0;
            width: 100%;
            border: ${default_border}px solid ${accent_color};
        }

        .nav_bar_styled {
            overflow: hidden;
            background-color: ${bg};
            position: fixed;
            top: 0;
            width: 100%;
            border: ${default_border}px solid ${accent_color};
        }

        .navbar a {
            float: left;
            display: block;
            color: ${ft_color};
            text-align: center;
            padding: 14px 16px;
            text-decoration: none;
        }

        .navbar a:hover {
            background-color: ${accent_color};
            color: ${bg};
        }

        .navbar a.active {
            background-color: ${accent_color};
            color: ${bg};
        }

        .navbar .icon {
            display: none;
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
