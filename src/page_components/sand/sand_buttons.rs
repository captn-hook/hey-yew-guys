
use yew::prelude::*;
use stylist::yew::styled_component;

pub enum Buttons {
    Rocket,
    Moon,
    Earth,
    Sun,
    Star,
}

pub struct SandButtons {
    buttons: Vec<Buttons>,
}
    
impl Component for SandButtons {
    type Message = Buttons;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            buttons: vec![
                Buttons::Rocket,
                Buttons::Moon,
                Buttons::Earth,
                Buttons::Sun,
                Buttons::Star,
            ],
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let rocketclick = ctx.link().callback(|e: MouseEvent| {
            //info!("rocket click");
            Buttons::Rocket
        });

        let moonclick = ctx.link().callback(|e: MouseEvent| {
            //info!("moon click");
            Buttons::Moon
        });

        let earthclick = ctx.link().callback(|e: MouseEvent| {
            //info!("earth click");
            Buttons::Earth
        });

        let sunclick = ctx.link().callback(|e: MouseEvent| {
            //info!("sun click");
            Buttons::Sun
        });

        let starclick = ctx.link().callback(|e: MouseEvent| {
            //info!("star click");
            Buttons::Star
        });

        html! {
            <div class="sand-buttons">
                <button class="sand-button" onclick={rocketclick}>{ "🚀" }</button>
                <button class="sand-button" onclick={moonclick}>{ "🌕" }</button>
                <button class="sand-button" onclick={earthclick}>{ "🌎" }</button>
                <button class="sand-button" onclick={sunclick}>{ "🌞" }</button>
                <button class="sand-button" onclick={starclick}>{ "⭐" }</button>    
            </div>
        }
    }
}       