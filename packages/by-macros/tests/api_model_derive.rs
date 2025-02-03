use by_macros::ApiModel;

#[derive(ApiModel, Default, Debug, Clone)]
pub enum ApiModelTest {
    Admin = 0,
    #[default]
    User = 1,
    Guest = 10,
}
