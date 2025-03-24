# Define a Structure

This document explains what elements should be included when configuring a Front Package and what functions each element performs.

## Directory Structure

The package structure is as follows:

```plaintext
├── components
│    ├── button
│    │    ├── mod.rs
│    ├── checkbox
│    │    ├── mod.rs
│    ├── selectbox
│          ├── mod.rs
├── layouts
│    ├── root_layout
│          ├── header.rs
│          └── mod.rs
│          └── footer.rs
│          └── side_bar.rs
├── pages
│    ├── main
│    │    ├── controller.rs
│    │    └── i18n.rs
│    │    └── mod.rs
│    │    └── page.rs
│    └── create
│          ├── controller.rs
│          └── i18n.rs
│          └── mod.rs
│          └── page.rs
├── services
│    ├── user_service.rs
│    └── popup_service.rs
├── utils
│    ├── api.rs
│    ├── time.rs
│    └── hash.rs
├── lib.rs
├── main.rs
├── routes.rs
└── config.rs
```

## Explanation of Structure

### Root Files

- **`main.rs`**  
  It is the entry point of the Dioxus app and is responsible for running the entire application by initializing the logger, registering services, setting themes, loading external resources, and configuring routing.

- **`lib.rs`**  
  The entry point that defines the structure of a Dioxus app and integrates and manages UI layouts, routes, pages, services, components, and utility modules.

  **Example:**

    ```
    pub mod config;
    pub mod pages;
    pub mod routes;

    pub mod service {
        pub mod user_service;
        pub mod popup_service;
    }

    pub mod utils {
        pub mod api;
        pub mod time;
        pub mod hash;
    }

    pub mod layouts {
        pub mod root_layout;
    }

    pub mod components {
        pub mod button;
        pub mod checkbox;
        pub mod selectbox;
    }
    ```

- **`routes.rs`**  
  Defines all page paths and language-specific routing structure within the app, and a router configuration for Dioxus that manages page navigation based on the /[:lang] namespace.

  **Example:**

    ```
    #[derive(Clone, Routable, Debug, PartialEq)]
    #[rustfmt::skip]
    pub enum Route {
        #[nest("/:lang")]
            #[layout(RootLayout)]
                #[route("/main")]
                MainPage { lang: Language },
            #[layout(RootLayout)]
                #[route("/create")]
                CreatePage { lang: Language },
            #[end_layout]
        #[end_nest]

        #[route("/:..route")]
        NotFoundPage { route: Vec<String> },
    }

    ```

- **`config.rs`**  
  A configuration module that initializes environment variable-based settings (Config) when the app runs and provides them in a globally accessible singleton form.

### components
- Create UI by organizing modules that can be commonly used for each page into a components directory.
- Components are configured by creating a directory with each component name and creating a mod.rs file inside that directory.

**Example:**

If the Components is defined as:
```
├── components
│    ├── button
│    │    ├── mod.rs
│    ├── checkbox
│    │    ├── mod.rs
│    ├── selectbox
│          ├── mod.rs
```

### layouts
- Organize common layouts for each page into the corresponding directories.
- Usually consists of header, footer, and sidebar areas.

**Example:**

If the Layouts is defined as:
```
├── layouts
│    ├── root_layout
│          ├── header.rs
│          └── mod.rs
│          └── footer.rs
│          └── side_bar.rs
```

### services
- Configure services for functions that operate commonly within a page.

#### step
The steps to configure and use the service are as follows: <br>
1. service file setting
- You can define a service file as follows:
``` popup_service.rs
#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct PopupService {
    pub id: Signal<Option<String>>,
}

impl PopupService {
    pub fn init() {
        let srv = Self {
            id: Signal::new(None),
        };
        use_context_provider(|| srv);
    }
}
``` 

2. Initialize in main.rs file
- Services are initialized globally in main.rs and are set up so that they can be used commonly by each page component.
- Service initialization is done as follows:
``` main.rs
use bdk::prelude::*;

pub mod service {
    pub mod popup_service;
}

fn main() {
    dioxus_logger::init(config::get().log_level).expect("failed to init logger");

    dioxus_aws::launch(App);
}

#[component]
fn App() -> Element {
    PopupService::init();

    rsx! {
        Router::<Route> {}
    }
}

```

3. accesssed via use_context
- In each page's controller.rs, services are accessed via use_context, allowing for seamless dependency injection and modular usage.
- Services can be accessed as follows:
``` controller.rs
#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    popup_service: PopupService,
}

impl Controller {
    pub fn new(
        lang: Language,
    ) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            popup_service: use_context(),
        };

        Ok(ctrl)
    }
}
```

### utils
- A space where utility functions that can be commonly used on each page are collected.
- Modularize and manage general logic such as time formatting (time.rs) or hash generation (hash.rs).

**Example:**

If the Utils is defined as:
```
├── utils
│    ├── api.rs
│    ├── time.rs
│    └── hash.rs
```

### pages
- Each page directory name must match the routing name.
- Each directory consists of controller.rs, i18n.rs, mod.rs, and page.rs.
- The page mainly utilizes the MVC pattern, the model is managed in the dto package, the ui is managed in the page.rs file, and the functional logic is managed in the controller.rs file.
- Components that are only used within a page must be used by configuring a separate components directory within the directory.

**Example:**

If the pages is defined as:
``` /main
├── main
│    ├── components
│    │    ├── mod.rs
│    │    ├── objective_quiz.rs
│    │    └── subjective_quiz.rs
│    ├── controller.rs
│    ├── i18n.rs
│    ├── mod.rs
│    └── page.rs
```

#### controller.rs
- Mainly creating functional logic.
- The pattern used is as follows:

**controller.rs**
``` controller.rs
#![allow(unused)]
use bdk::prelude::*;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    count: Signal<i64>,
    lang: Language,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let count = use_signal(|| 0);
        let ctrl = Self { lang, count };
        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn add_count(&mut self) {
        let mut c = count();
        c += 1;
        count.set(c);
    }

    pub fn get_count(&mut self) {
        count()
    }
}

```

**page.rs**
``` page.rs
use bdk::prelude::*;

#[component]
pub fn MainPage(lang: Language) -> Element {
    let ctrl = Controller::new(lang);
    let count = ctrl.get_count();

    rsx! {
        div {
            "main page"
        }
    }
}
```

#### i18n.rs
- Translate configuration using the translate macro, which is an internally implemented macro.
- Basically, ko and en are supported in two versions, and the two versions are described in the i18n.rs file when configuring the UI.
- Examples of usage are as follows.

**i18n.rs**
``` i18n.rs
use bdk::prelude::*;

translate! {
    Translate;

    text: {
        ko: "메인 페이지",
        en: "Main Page"
    }
}

```

**page.rs**
``` page.rs
use bdk::prelude::*;

#[component]
pub fn MainPage(lang: Language) -> Element {
    let ctrl = Controller::new(lang);
    let tr = translate(&lang);

    let count = ctrl.get_count();
    let text = tr.text;

    rsx! {
        div {
            "main page"
        }
    }
}
```

#### page.rs
- page.rs is defined around UI composition and screen elements shown to the user, rather than business logic.
- Business logic is configured in the controller.rs file, and translation-related parts are configured in the i18n.rs file.