#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Page {
    // Guide ("Guide" mode)
    #[default]
    Introduction,
    Roadmap,
    Community,

    // Documentation ("Documentation" mode)
    Overview,
    Architecture,
    ProjectStructure,
    Customizations,
    BasicSizing,
    Typography,
    Layout,

    // Components -> Atoms
    Text,
    Icon,
    Divider,
    Button,
    Shapes,

    // Components -> Containers
    VStack,
    HStack,
    ZStack,
    Overlay,
    ScrollView,
    Card,

    // Components -> Navigation
    Sidebar,
    Tabbar,
    Modal,
    NavigationSplit,
    Section,

    // API Schema
    ApiSchema,

    // Showcase ("Components" mode)
    // Note: These map to plural strings in sidebar: "Buttons", "Inputs"
    ShowcaseButtons,
    ShowcaseInputs,
    ShowcaseToggles,
    ShowcaseSliders,
    ShowcasePickers,

    // Hooks ("Hooks" mode)
    UseState,
    UseEffect,
    UseMemo,
    UseCallback,

    // Core Services
    PeakDB,
    PeakCloud,

    // Applications
    PeakDesktop,
    PeakOSCore,

    // Settings ("Settings" mode)
    Appearance,
    Scaling,
    Shortcuts,
    About,
    Updates,

    // Fallback
    Unknown(String),
}

impl ToString for Page {
    fn to_string(&self) -> String {
        match self {
            Page::Introduction => "Introduction".to_string(),
            Page::Roadmap => "Roadmap".to_string(),
            Page::Community => "Community".to_string(),
            Page::Overview => "Overview".to_string(),
            Page::Architecture => "Architecture".to_string(),
            Page::ProjectStructure => "Project Structure".to_string(),
            Page::Customizations => "Customizations".to_string(),
            Page::BasicSizing => "Basic Sizing".to_string(),
            Page::Typography => "Typography".to_string(),
            Page::Layout => "Layout".to_string(),

            Page::Text => "Text".to_string(),
            Page::Icon => "Icon".to_string(),
            Page::Divider => "Divider".to_string(),
            Page::Button => "Button".to_string(),
            Page::Shapes => "Shapes".to_string(),
            Page::VStack => "VStack".to_string(),
            Page::HStack => "HStack".to_string(),
            Page::ZStack => "ZStack".to_string(),
            Page::Overlay => "Overlay".to_string(),
            Page::ScrollView => "ScrollView".to_string(),
            Page::Card => "Card".to_string(),
            Page::Sidebar => "Sidebar".to_string(),
            Page::Tabbar => "Tabbar".to_string(),
            Page::Modal => "Modal".to_string(),
            Page::NavigationSplit => "NavigationSplit".to_string(),
            Page::Section => "Section".to_string(),

            Page::ApiSchema => "API Schema".to_string(),

            Page::ShowcaseButtons => "Buttons".to_string(),
            Page::ShowcaseInputs => "Inputs".to_string(),
            Page::ShowcaseToggles => "Toggles".to_string(),
            Page::ShowcaseSliders => "Sliders".to_string(),
            Page::ShowcasePickers => "Pickers".to_string(),

            Page::UseState => "use_state".to_string(),
            Page::UseEffect => "use_effect".to_string(),
            Page::UseMemo => "use_memo".to_string(),
            Page::UseCallback => "use_callback".to_string(),

            Page::PeakDB => "PeakDB".to_string(),
            Page::PeakCloud => "PeakCloud".to_string(),
            Page::PeakDesktop => "PeakDesktop".to_string(),
            Page::PeakOSCore => "PeakOS Core".to_string(),

            Page::Appearance => "Appearance".to_string(),
            Page::Scaling => "Scaling".to_string(),
            Page::Shortcuts => "Shortcuts".to_string(),
            Page::About => "About".to_string(),
            Page::Updates => "Updates".to_string(),

            Page::Unknown(s) => s.clone(),
        }
    }
}

impl From<String> for Page {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Introduction" => Page::Introduction,
            "Roadmap" => Page::Roadmap,
            "Community" => Page::Community,
            "Overview" => Page::Overview,
            "Architecture" => Page::Architecture,
            "Project Structure" => Page::ProjectStructure,
            "Customizations" => Page::Customizations,
            "Basic Sizing" => Page::BasicSizing,
            "Typography" => Page::Typography,
            "Layout" => Page::Layout,

            "Text" => Page::Text,
            "Icon" => Page::Icon,
            "Divider" => Page::Divider,
            "Button" => Page::Button,
            "Shapes" => Page::Shapes,

            "VStack" => Page::VStack,
            "HStack" => Page::HStack,
            "ZStack" => Page::ZStack,
            "Overlay" => Page::Overlay,
            "ScrollView" => Page::ScrollView,
            "Card" => Page::Card,

            "Sidebar" => Page::Sidebar,
            "Tabbar" => Page::Tabbar,
            "Modal" => Page::Modal,
            "NavigationSplit" => Page::NavigationSplit,
            "Section" => Page::Section,

            "API Schema" => Page::ApiSchema,

            "Buttons" => Page::ShowcaseButtons,
            "Inputs" => Page::ShowcaseInputs,
            "Toggles" => Page::ShowcaseToggles,
            "Sliders" => Page::ShowcaseSliders,
            "Pickers" => Page::ShowcasePickers,

            "use_state" => Page::UseState,
            "use_effect" => Page::UseEffect,
            "use_memo" => Page::UseMemo,
            "use_callback" => Page::UseCallback,

            "PeakDB" => Page::PeakDB,
            "PeakCloud" => Page::PeakCloud,
            "PeakDesktop" => Page::PeakDesktop,
            "PeakOS Core" => Page::PeakOSCore,

            "Appearance" => Page::Appearance,
            "Scaling" => Page::Scaling,
            "Shortcuts" => Page::Shortcuts,
            "About" => Page::About,
            "Updates" => Page::Updates,

            _ => Page::Unknown(s),
        }
    }
}
