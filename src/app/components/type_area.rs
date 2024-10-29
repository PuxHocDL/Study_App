use leptos::{html::Input, *};

const INPUT_AREA_CLASS: &str =
    "h-24 w-full fixed bottom-0 flex justify-center items-center p-5 border-t";
const INPUT_AREA_CLASS_COLOURS: &str = "bg-inp";

const TEXT_AREA_CLASS: &str = "w-2/3 p-4 border-2 input-field";
const TEXT_AREA_CLASS_COLOURS: &str = "bg-txt";

const BUTTON_CLASS: &str = "h-full p-4 cursor-pointer";
const BUTTON_CLASS_COLOURS: &str =
    "bg-gradient-to-r from-button_d to-button_l text-white hover:from-hover_d hover:to-hover_l";

#[component]
pub fn TypeArea(send: Action<String, Result<(), ServerFnError>>) -> impl IntoView {
    let type_area_class = format!("{INPUT_AREA_CLASS} {INPUT_AREA_CLASS_COLOURS}");
    let text_area_class = format!("{TEXT_AREA_CLASS} {TEXT_AREA_CLASS_COLOURS}");
    let button_class = format!("{BUTTON_CLASS} {BUTTON_CLASS_COLOURS}");

    let input_ref = create_node_ref::<Input>();
    view! {
        <div class=type_area_class>
            <form
                class="w-full flex justify-center items-center gap-4"
                on:submit=move |ev| {
                    ev.prevent_default();
                    let input = input_ref.get().expect("input to exist");
                    send.dispatch(input.value());
                    input.set_value("");
                }
            >

                <input class=text_area_class type="text" node_ref=input_ref/>
                <button class=button_class type="submit">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="w-6 h-6"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M4.5 12h15m0 0l-6.75-6.75M19.5 12l-6.75 6.75"
                        ></path>
                    </svg>
                </button>
            </form>
        </div>
    }
}