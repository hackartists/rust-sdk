#[derive(Debug, Clone)]
pub struct ColorTheme {
    pub background: String,
    pub icon: IconColorTheme,
    pub button: ButtonColorTheme,
    pub text: TextColorTheme,
    pub card: CardColorTheme,
    pub popup: PopupColorTheme,
    pub input: InputColorTheme,
    pub services: ServiceColorTheme,
}

impl Default for ColorTheme {
    fn default() -> Self {
        ColorTheme {
            background: "#2C2E42".to_string(),
            icon: IconColorTheme::default(),
            button: ButtonColorTheme::default(),
            text: TextColorTheme::default(),
            card: CardColorTheme::default(),
            popup: PopupColorTheme::default(),
            input: InputColorTheme::default(),
            services: ServiceColorTheme::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServiceColorTheme {
    pub kakao: String,
}

impl Default for ServiceColorTheme {
    fn default() -> Self {
        ServiceColorTheme {
            kakao: "#F7E111".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IconColorTheme {
    pub primary: String,
}

impl Default for IconColorTheme {
    fn default() -> Self {
        IconColorTheme {
            primary: "#8588AB".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ButtonColorTheme {
    pub primary: String,
    pub secondary: String,
}

impl Default for ButtonColorTheme {
    fn default() -> Self {
        ButtonColorTheme {
            primary: "#363952".to_string(),
            secondary: "#424563".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextColorTheme {
    pub primary: String,
    pub secondary: String,
    pub disabled: String,
}

impl Default for TextColorTheme {
    fn default() -> Self {
        TextColorTheme {
            primary: "#FFFFFF".to_string(),
            secondary: "#ADBCD7".to_string(),
            disabled: "#404761".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CardColorTheme {
    pub primary: String,
    pub secondary: String,
}

impl Default for CardColorTheme {
    fn default() -> Self {
        CardColorTheme {
            primary: "#212231".to_string(),
            secondary: "#292B3D".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PopupColorTheme {
    pub background: String,
}

impl Default for PopupColorTheme {
    fn default() -> Self {
        PopupColorTheme {
            background: "#2E3045".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputColorTheme {
    pub primary: String,
}

impl Default for InputColorTheme {
    fn default() -> Self {
        InputColorTheme {
            primary: "#212231".to_string(),
        }
    }
}
