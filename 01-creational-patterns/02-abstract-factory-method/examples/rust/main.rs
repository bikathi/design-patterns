pub trait Button {
    fn click(&self);
}

pub trait Checkbox {
    fn switched(&self);
}

// Abstract class v1 -> generics to prevent dynamic dispatch
pub trait AbstractGUIFactory {
    type B: Button;
    type C: Checkbox;

    fn create_button(&self) -> Self::B;
    fn create_checkbox(&self) -> Self::C;
}

// Abstract v2 -> dyn keyword to reduce binary bloat but uses dynamic dispatch
/*
trait AbstractGUIFactoryV2 {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Boxy<dyn Checkbox>;
}
*/

// MacOS example
mod macos {
    pub struct MacOSButton;
    pub struct MacOSCheckbox;
    pub struct MacOSGUIFactory;

    impl super::Button for MacOSButton {
        fn click(&self) {
            println!("MacOS Button clicked!");
        }
    }

    impl super::Checkbox for MacOSCheckbox {
        fn switched(&self) {
            println!("MacOS checkbox switched!");
        }
    }

    impl super::AbstractGUIFactory for MacOSGUIFactory {
        // type aliases for this struct
        type B = MacOSButton;
        type C = MacOSCheckbox;

        fn create_button(&self) -> Self::B {
            MacOSButton
        }

        fn create_checkbox(&self) -> Self::C {
            MacOSCheckbox
        }
    }
}

mod windows {
    pub struct WindowsButton;
    pub struct WindowsCheckbox;
    pub struct WindowsGUIFactory;

    impl super::Button for WindowsButton {
        fn click(&self) {
            println!("Windows Button clicked!");
        }
    }

    impl super::Checkbox for WindowsCheckbox {
        fn switched(&self) {
            println!("Windows checkbox switched!");
        }
    }

    impl super::AbstractGUIFactory for WindowsGUIFactory {
        // type aliases for this struct
        type B = WindowsButton;
        type C = WindowsCheckbox;

        fn create_button(&self) -> Self::B {
            WindowsButton
        }

        fn create_checkbox(&self) -> Self::C {
            WindowsCheckbox
        }
    }
}

fn main() {
    let on_windows = false;

    if on_windows {
        render(windows::WindowsGUIFactory);
    } else {
        render(macos::MacOSGUIFactory)
    }
}

fn render<R>(r: R)
where
    R: AbstractGUIFactory,
{
    let button = r.create_button();
    let checkbox = r.create_checkbox();

    button.click();
    checkbox.switched();
}
