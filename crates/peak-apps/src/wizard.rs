use iced::Task;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardStep {
    Welcome,
    Identity, // Step 2a: Name & Username
    Security, // Step 2b: Password & Hint
    WifiConnect,
    ThemeSelection,
    Complete,
}

#[derive(Debug, Clone)]
pub struct WizardState {
    pub current_step: WizardStep,
    pub full_name_input: String,
    pub username_input: String,
    pub password_input: String,
    pub password_confirm_input: String,
    pub password_hint_input: String,
    pub selected_network: Option<String>,
    pub wifi_password_input: String,
    pub selected_avatar: Option<String>,
    pub selected_mode: Option<String>, // "desktop", "mobile", "tv", "console"
    pub error_message: Option<String>,
}

impl Default for WizardState {
    fn default() -> Self {
        Self {
            current_step: WizardStep::Welcome,
            full_name_input: String::new(),
            username_input: String::new(),
            password_input: String::new(),
            password_confirm_input: String::new(),
            password_hint_input: String::new(),
            selected_network: None,
            wifi_password_input: String::new(),
            selected_avatar: None,
            selected_mode: None,
            error_message: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum WizardMessage {
    NextStep,
    PrevStep,
    UpdateFullName(String),
    UpdateUsername(String),
    UpdatePassword(String),
    UpdatePasswordConfirm(String),
    UpdatePasswordHint(String),
    SelectNetwork(String),
    UpdateWifiPassword(String),
    SelectAvatar(String),
    SelectMode(String),
    CompleteSetup,
}

pub fn update(state: &mut WizardState, message: WizardMessage) -> Task<WizardMessage> {
    match message {
        WizardMessage::NextStep => {
            // Clear previous errors
            state.error_message = None;

            match state.current_step {
                WizardStep::Welcome => state.current_step = WizardStep::Identity,
                WizardStep::Identity => {
                    if state.full_name_input.trim().is_empty() {
                        state.error_message = Some("Full Name is required.".to_string());
                    } else if state.username_input.trim().is_empty() {
                        state.error_message = Some("Account Name is required.".to_string());
                    } else {
                        state.current_step = WizardStep::Security;
                    }
                }
                WizardStep::Security => {
                    if state.password_input.is_empty() {
                        state.error_message = Some("Password is required.".to_string());
                    } else if state.password_input != state.password_confirm_input {
                        state.error_message = Some("Passwords do not match.".to_string());
                    } else {
                        state.current_step = WizardStep::WifiConnect;
                    }
                }
                WizardStep::WifiConnect => state.current_step = WizardStep::ThemeSelection,
                WizardStep::ThemeSelection => {
                    if state.selected_mode.is_none() {
                        state.error_message = Some("Please select an experience mode.".to_string());
                    } else {
                        state.current_step = WizardStep::Complete;
                    }
                }
                WizardStep::Complete => {} // Handled by parent
            }
        }
        WizardMessage::PrevStep => {
            state.error_message = None; // Clear errors on back
            match state.current_step {
                WizardStep::Welcome => {}
                WizardStep::Identity => state.current_step = WizardStep::Welcome,
                WizardStep::Security => state.current_step = WizardStep::Identity,
                WizardStep::WifiConnect => state.current_step = WizardStep::Security, // Fix back nav
                WizardStep::ThemeSelection => state.current_step = WizardStep::WifiConnect,
                WizardStep::Complete => state.current_step = WizardStep::ThemeSelection,
            }
        }
        WizardMessage::UpdateFullName(s) => {
            state.full_name_input = s;
            state.error_message = None;
        }
        WizardMessage::UpdateUsername(s) => {
            state.username_input = s;
            state.error_message = None;
        }
        WizardMessage::UpdatePassword(s) => {
            state.password_input = s;
            state.error_message = None;
        }
        WizardMessage::UpdatePasswordConfirm(s) => {
            state.password_confirm_input = s;
            state.error_message = None;
        }
        WizardMessage::UpdatePasswordHint(s) => {
            state.password_hint_input = s;
            state.error_message = None;
        }
        WizardMessage::SelectNetwork(s) => state.selected_network = Some(s),
        WizardMessage::UpdateWifiPassword(s) => state.wifi_password_input = s,
        WizardMessage::SelectAvatar(s) => state.selected_avatar = Some(s),
        WizardMessage::SelectMode(s) => {
            state.selected_mode = Some(s);
            state.error_message = None;
        }
        WizardMessage::CompleteSetup => {} // Handled by parent
    }
    Task::none()
}
