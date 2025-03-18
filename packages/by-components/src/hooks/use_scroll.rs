use dioxus::prelude::*;

pub struct UseScroll {
    pub task: CopyValue<Task>,
    pub callback: Callback<(), Task>,
}

pub enum ScrollDirection {
    Up,
    Down,
}

pub fn use_scroll<F>(mut callback: F)
where
    F: FnMut(f64, f64, bool, ScrollDirection) + 'static + Copy,
{
    let callback = use_callback(move |_| {
        let script = r#"
            window.addEventListener('scroll', () => {
                let scrollY = window.scrollY;
                let scrollX = window.scrollX;
                let scrollHeight = document.documentElement.scrollHeight-1;
                let innerHeight = window.innerHeight;

                let isBottom = (scrollY + innerHeight) >= scrollHeight;

                dioxus.send(`${scrollY},${scrollX},${isBottom}`);
            });
        "#;
        let mut eval = document::eval(script);

        spawn(async move {
            let mut y = 0.0;

            loop {
                let pos = eval.recv::<String>().await.unwrap_or_default();
                let mut pos = pos.split(',');
                let new_y = pos
                    .next()
                    .unwrap_or_default()
                    .parse::<f64>()
                    .unwrap_or_default();
                let x = pos
                    .next()
                    .unwrap_or_default()
                    .parse::<f64>()
                    .unwrap_or_default();

                let is_bottom = pos
                    .next()
                    .unwrap_or_default()
                    .parse::<bool>()
                    .unwrap_or_default();
                let direction = if new_y > y {
                    ScrollDirection::Down
                } else {
                    ScrollDirection::Up
                };

                y = new_y;

                callback(x, y, is_bottom, direction);
            }
        })
    });

    let task = use_hook(|| CopyValue::new(callback(())));

    UseScroll { task, callback };
}
