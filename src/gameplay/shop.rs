use super::generators::{GeneratorState, GeneratorType};
use super::progression::ArcaneProgress;
use super::resources::SecondaryResources;
use super::state::GameState;
use super::synergies::SynergyState;
use super::transcendence::TranscendenceState;
use crate::orb::types::{EquippedOrb, Orb, OrbType};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// ========== DATA TYPES ==========

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShopItemId {
    ArcaneBiscuit,
    VoidTea,
    CosmicPretzel,
    GlowingBerries,
    FocusedMind,
    DeepContemplation,
    ArcaneAmplifier,
    CrystalResonance,
    GentleScaling,
    ObsidianOrb,
    MercuryOrb,
    GalaxyOrb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopCategory {
    Snacks,
    Upgrades,
    Generators,
    OrbCollection,
}

impl ShopCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Snacks => "Snacks",
            Self::Upgrades => "Upgrades",
            Self::Generators => "Generators",
            Self::OrbCollection => "Orb Collection",
        }
    }

    pub const ALL: [ShopCategory; 4] = [
        ShopCategory::Snacks,
        ShopCategory::Upgrades,
        ShopCategory::Generators,
        ShopCategory::OrbCollection,
    ];
}

#[derive(Debug, Clone)]
pub struct ShopItem {
    pub id: ShopItemId,
    pub category: ShopCategory,
    pub name: &'static str,
    pub description: &'static str,
    pub cost: u64,
}

#[derive(Resource)]
pub struct ShopCatalog {
    pub items: Vec<ShopItem>,
}

impl Default for ShopCatalog {
    fn default() -> Self {
        Self {
            items: vec![
                // Snacks
                ShopItem {
                    id: ShopItemId::ArcaneBiscuit,
                    category: ShopCategory::Snacks,
                    name: "Arcane Biscuit",
                    description: "Tastes like contemplation and oats. (+0.1 efficiency)",
                    cost: 20,
                },
                ShopItem {
                    id: ShopItemId::VoidTea,
                    category: ShopCategory::Snacks,
                    name: "Void Tea",
                    description: "Brewed from the absence of tea leaves. (+0.25 efficiency)",
                    cost: 50,
                },
                ShopItem {
                    id: ShopItemId::CosmicPretzel,
                    category: ShopCategory::Snacks,
                    name: "Cosmic Pretzel",
                    description: "Twisted by gravitational forces of pure thought. (+0.5 efficiency)",
                    cost: 100,
                },
                ShopItem {
                    id: ShopItemId::GlowingBerries,
                    category: ShopCategory::Snacks,
                    name: "Glowing Berries",
                    description: "Harvested from bushes that dream of being stars. (+1.0 efficiency)",
                    cost: 200,
                },
                // Upgrades
                ShopItem {
                    id: ShopItemId::FocusedMind,
                    category: ShopCategory::Upgrades,
                    name: "Focused Mind",
                    description: "Sharpen your mental lens. (+20% wisdom speed)",
                    cost: 30,
                },
                ShopItem {
                    id: ShopItemId::DeepContemplation,
                    category: ShopCategory::Upgrades,
                    name: "Deep Contemplation",
                    description: "Think thoughts about thoughts. (+50% wisdom speed)",
                    cost: 75,
                },
                ShopItem {
                    id: ShopItemId::ArcaneAmplifier,
                    category: ShopCategory::Upgrades,
                    name: "Arcane Amplifier",
                    description: "Focuses the arcane flow. (+5 AFP per truth)",
                    cost: 40,
                },
                ShopItem {
                    id: ShopItemId::CrystalResonance,
                    category: ShopCategory::Upgrades,
                    name: "Crystal Resonance",
                    description: "The orb hums in harmony. (+10 AFP per truth)",
                    cost: 80,
                },
                ShopItem {
                    id: ShopItemId::GentleScaling,
                    category: ShopCategory::Upgrades,
                    name: "Gentle Scaling",
                    description: "Softens the rising tide of wisdom. (Scaling 1.1x \u{2192} 1.07x)",
                    cost: 60,
                },
                // Orb Collection
                ShopItem {
                    id: ShopItemId::ObsidianOrb,
                    category: ShopCategory::OrbCollection,
                    name: "Obsidian Orb",
                    description: "Forged in forgotten volcanoes. (+0.3 efficiency, +5 AFP/truth)",
                    cost: 150,
                },
                ShopItem {
                    id: ShopItemId::MercuryOrb,
                    category: ShopCategory::OrbCollection,
                    name: "Mercury Orb",
                    description: "Liquid metal in a sphere of pure intent. (+40% wisdom speed)",
                    cost: 300,
                },
                ShopItem {
                    id: ShopItemId::GalaxyOrb,
                    category: ShopCategory::OrbCollection,
                    name: "Galaxy Orb",
                    description: "Contains an entire galaxy. (Scaling -0.03)",
                    cost: 500,
                },
            ],
        }
    }
}

#[derive(Resource)]
pub struct PurchaseTracker {
    pub purchased: HashSet<ShopItemId>,
    pub efficiency_bonus: f32,
    pub wisdom_speed_bonus: f32,
    pub afp_bonus: u32,
    pub scaling_factor: f32,
}

impl Default for PurchaseTracker {
    fn default() -> Self {
        Self {
            purchased: HashSet::new(),
            efficiency_bonus: 0.0,
            wisdom_speed_bonus: 1.0,
            afp_bonus: 0,
            scaling_factor: 1.1,
        }
    }
}

impl PurchaseTracker {
    pub fn recalculate(&mut self, equipped: OrbType) {
        self.efficiency_bonus = 0.0;
        self.wisdom_speed_bonus = 1.0;
        self.afp_bonus = 0;
        self.scaling_factor = 1.1;

        for item in &self.purchased {
            match item {
                ShopItemId::ArcaneBiscuit => self.efficiency_bonus += 0.1,
                ShopItemId::VoidTea => self.efficiency_bonus += 0.25,
                ShopItemId::CosmicPretzel => self.efficiency_bonus += 0.5,
                ShopItemId::GlowingBerries => self.efficiency_bonus += 1.0,
                ShopItemId::FocusedMind => self.wisdom_speed_bonus += 0.2,
                ShopItemId::DeepContemplation => self.wisdom_speed_bonus += 0.5,
                ShopItemId::ArcaneAmplifier => self.afp_bonus += 5,
                ShopItemId::CrystalResonance => self.afp_bonus += 10,
                ShopItemId::GentleScaling => self.scaling_factor = 1.07,
                _ => {}
            }
        }

        // Equipped orb bonuses (applied after shop item bonuses)
        match equipped {
            OrbType::Crystal => {}
            OrbType::Obsidian => {
                self.efficiency_bonus += 0.3;
                self.afp_bonus += 5;
            }
            OrbType::Mercury => {
                self.wisdom_speed_bonus += 0.4;
            }
            OrbType::Galaxy => {
                self.scaling_factor -= 0.03;
            }
        }
    }
}

// ========== UI COMPONENTS ==========

#[derive(Component)]
pub struct ShopPanel;

#[derive(Component)]
pub struct ShopItemList;

#[derive(Component)]
pub struct CategoryTab(pub ShopCategory);

#[derive(Component)]
pub struct BuyButton(pub ShopItemId);

#[derive(Component)]
pub struct EquipButton(pub OrbType);

#[derive(Component)]
pub struct BuyGeneratorButton(pub GeneratorType);

#[derive(Component)]
pub struct ShopAfpText;

#[derive(Resource)]
pub struct SelectedCategory(pub ShopCategory);

fn shop_item_to_orb_type(id: ShopItemId) -> Option<OrbType> {
    match id {
        ShopItemId::ObsidianOrb => Some(OrbType::Obsidian),
        ShopItemId::MercuryOrb => Some(OrbType::Mercury),
        ShopItemId::GalaxyOrb => Some(OrbType::Galaxy),
        _ => None,
    }
}

// ========== SYSTEMS ==========

pub fn toggle_shop(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyB) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::ShopOpen),
            GameState::ShopOpen => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

pub fn open_shop(
    mut commands: Commands,
    catalog: Res<ShopCatalog>,
    tracker: Res<PurchaseTracker>,
    progress: Res<ArcaneProgress>,
    equipped: Res<EquippedOrb>,
    generators: Res<GeneratorState>,
    synergies: Res<SynergyState>,
    transcendence: Res<TranscendenceState>,
    resources: Res<SecondaryResources>,
) {
    commands.insert_resource(SelectedCategory(ShopCategory::Snacks));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ShopPanel,
        ))
        .with_children(|backdrop| {
            backdrop
                .spawn((
                    Node {
                        width: Val::Px(620.0),
                        max_height: Val::Percent(85.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(24.0)),
                        row_gap: Val::Px(12.0),
                        overflow: Overflow::scroll_y(),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.08, 0.06, 0.14, 0.95)),
                ))
                .with_children(|panel| {
                    // Header row
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|header| {
                            header.spawn((
                                Text::new("Arcane Emporium"),
                                TextFont {
                                    font_size: 28.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 0.85, 0.4)),
                            ));
                            header.spawn((
                                Text::new(format!("AFP: {}", progress.focus_points)),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 0.85, 0.4)),
                                ShopAfpText,
                            ));
                        });

                    // Divider
                    panel.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(1.0, 0.85, 0.4, 0.3)),
                    ));

                    // Category tabs
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|tabs| {
                            for cat in ShopCategory::ALL {
                                let is_active = cat == ShopCategory::Snacks;
                                let bg = if is_active {
                                    Color::srgba(1.0, 0.85, 0.4, 0.15)
                                } else {
                                    Color::srgba(0.3, 0.25, 0.4, 0.3)
                                };
                                let text_color = if is_active {
                                    Color::srgb(1.0, 0.85, 0.4)
                                } else {
                                    Color::srgba(0.6, 0.55, 0.7, 0.6)
                                };

                                tabs.spawn((
                                    Button,
                                    Node {
                                        padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        ..default()
                                    },
                                    BackgroundColor(bg),
                                    CategoryTab(cat),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new(cat.label()),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(text_color),
                                    ));
                                });
                            }
                        });

                    // Divider
                    panel.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(1.0, 0.85, 0.4, 0.15)),
                    ));

                    // Item list container
                    panel
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(8.0),
                                ..default()
                            },
                            ShopItemList,
                        ))
                        .with_children(|list| {
                            spawn_items(
                                list,
                                &catalog,
                                &tracker,
                                &progress,
                                ShopCategory::Snacks,
                                &equipped,
                                &generators,
                                &synergies,
                                &transcendence,
                                &resources,
                            );
                        });

                    // Footer
                    panel.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            margin: UiRect::top(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(1.0, 0.85, 0.4, 0.15)),
                    ));
                    panel.spawn((
                        Text::new("Press [B] to close"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.6, 0.55, 0.7, 0.5)),
                    ));
                });
        });
}

pub fn close_shop(mut commands: Commands, panels: Query<Entity, With<ShopPanel>>) {
    for entity in &panels {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<SelectedCategory>();
}

pub fn handle_category_click(
    interactions: Query<(&Interaction, &CategoryTab), Changed<Interaction>>,
    mut selected: ResMut<SelectedCategory>,
) {
    for (interaction, tab) in &interactions {
        if *interaction == Interaction::Pressed && selected.0 != tab.0 {
            selected.0 = tab.0;
        }
    }
}

pub fn rebuild_item_list(
    mut commands: Commands,
    selected: Res<SelectedCategory>,
    catalog: Res<ShopCatalog>,
    tracker: Res<PurchaseTracker>,
    progress: Res<ArcaneProgress>,
    equipped: Res<EquippedOrb>,
    generators: Res<GeneratorState>,
    synergies: Res<SynergyState>,
    transcendence: Res<TranscendenceState>,
    resources: Res<SecondaryResources>,
    list_query: Query<Entity, With<ShopItemList>>,
    tab_query: Query<(&CategoryTab, &Children)>,
    mut text_query: Query<&mut TextColor>,
) {
    if !selected.is_changed()
        && !equipped.is_changed()
        && !tracker.is_changed()
        && !generators.is_changed()
    {
        return;
    }

    // Update tab text colors when category changes
    if selected.is_changed() {
        for (tab, children) in &tab_query {
            let is_active = tab.0 == selected.0;
            for child in children.iter() {
                if let Ok(mut tc) = text_query.get_mut(child) {
                    tc.0 = if is_active {
                        Color::srgb(1.0, 0.85, 0.4)
                    } else {
                        Color::srgba(0.6, 0.55, 0.7, 0.6)
                    };
                }
            }
        }
    }

    // Despawn old items and respawn
    for list_entity in &list_query {
        commands.entity(list_entity).despawn_related::<Children>();
        commands
            .entity(list_entity)
            .with_children(|list: &mut ChildSpawnerCommands| {
                spawn_items(
                    list,
                    &catalog,
                    &tracker,
                    &progress,
                    selected.0,
                    &equipped,
                    &generators,
                    &synergies,
                    &transcendence,
                    &resources,
                );
            });
    }
}

pub fn update_tab_backgrounds(
    selected: Res<SelectedCategory>,
    mut tabs: Query<(&CategoryTab, &mut BackgroundColor)>,
) {
    if !selected.is_changed() {
        return;
    }
    for (tab, mut bg) in &mut tabs {
        *bg = if tab.0 == selected.0 {
            BackgroundColor(Color::srgba(1.0, 0.85, 0.4, 0.15))
        } else {
            BackgroundColor(Color::srgba(0.3, 0.25, 0.4, 0.3))
        };
    }
}

pub fn handle_buy_click(
    interactions: Query<(&Interaction, &BuyButton), Changed<Interaction>>,
    catalog: Res<ShopCatalog>,
    mut tracker: ResMut<PurchaseTracker>,
    mut progress: ResMut<ArcaneProgress>,
    equipped: Res<EquippedOrb>,
) {
    for (interaction, button) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if tracker.purchased.contains(&button.0) {
            continue;
        }

        let Some(item) = catalog.items.iter().find(|i| i.id == button.0) else {
            continue;
        };

        if progress.focus_points < item.cost {
            continue;
        }

        // Purchase!
        progress.focus_points -= item.cost;
        tracker.purchased.insert(button.0);

        // Apply orb unlocks directly
        match button.0 {
            ShopItemId::ObsidianOrb => {
                if !progress.unlocked_orbs.contains(&OrbType::Obsidian) {
                    progress.unlocked_orbs.push(OrbType::Obsidian);
                }
            }
            ShopItemId::MercuryOrb => {
                if !progress.unlocked_orbs.contains(&OrbType::Mercury) {
                    progress.unlocked_orbs.push(OrbType::Mercury);
                }
            }
            ShopItemId::GalaxyOrb => {
                if !progress.unlocked_orbs.contains(&OrbType::Galaxy) {
                    progress.unlocked_orbs.push(OrbType::Galaxy);
                }
            }
            _ => {}
        }

        tracker.recalculate(equipped.0);
    }
}

pub fn update_shop_buttons(
    tracker: Res<PurchaseTracker>,
    progress: Res<ArcaneProgress>,
    catalog: Res<ShopCatalog>,
    mut buttons: Query<(&BuyButton, &mut BackgroundColor, &Children)>,
    mut texts: Query<&mut Text>,
) {
    for (button, mut bg, children) in &mut buttons {
        let owned = tracker.purchased.contains(&button.0);
        let affordable = catalog
            .items
            .iter()
            .find(|i| i.id == button.0)
            .map(|i| progress.focus_points >= i.cost)
            .unwrap_or(false);

        let color = if owned {
            Color::srgba(0.2, 0.5, 0.25, 0.6)
        } else if affordable {
            Color::srgba(1.0, 0.85, 0.4, 0.9)
        } else {
            Color::srgba(0.3, 0.25, 0.4, 0.5)
        };

        *bg = BackgroundColor(color);

        for child in children.iter() {
            if let Ok(mut text) = texts.get_mut(child) {
                if owned {
                    **text = "Owned".to_string();
                } else if let Some(item) = catalog.items.iter().find(|i| i.id == button.0) {
                    **text = format!("{} AFP", item.cost);
                }
            }
        }
    }
}

pub fn update_shop_afp(
    progress: Res<ArcaneProgress>,
    mut query: Query<&mut Text, With<ShopAfpText>>,
) {
    for mut text in &mut query {
        **text = format!("AFP: {}", format_afp(progress.focus_points));
    }
}

pub fn handle_equip_click(
    interactions: Query<(&Interaction, &EquipButton), Changed<Interaction>>,
    mut equipped: ResMut<EquippedOrb>,
    mut tracker: ResMut<PurchaseTracker>,
    mut orb_query: Query<&mut Orb>,
) {
    for (interaction, button) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if equipped.0 == button.0 {
            continue;
        }

        equipped.0 = button.0;

        for mut orb in &mut orb_query {
            orb.orb_type = button.0;
        }

        tracker.recalculate(button.0);
    }
}

// ========== UI HELPERS ==========

fn spawn_items(
    parent: &mut ChildSpawnerCommands,
    catalog: &ShopCatalog,
    tracker: &PurchaseTracker,
    progress: &ArcaneProgress,
    category: ShopCategory,
    equipped: &EquippedOrb,
    generators: &GeneratorState,
    synergies: &SynergyState,
    transcendence: &TranscendenceState,
    resources: &SecondaryResources,
) {
    // Generator tab has its own rendering
    if category == ShopCategory::Generators {
        spawn_generator_items(parent, generators, synergies, progress, transcendence, resources);
        return;
    }

    // Crystal orb row (always available at top of Orb Collection)
    if category == ShopCategory::OrbCollection {
        let is_equipped = equipped.0 == OrbType::Crystal;
        spawn_orb_row(
            parent,
            "Crystal Orb",
            "Your trusty starter orb. Reliable and familiar.",
            OrbType::Crystal,
            is_equipped,
        );
    }

    let items: Vec<&ShopItem> = catalog
        .items
        .iter()
        .filter(|i| i.category == category)
        .collect();

    if items.is_empty() && category != ShopCategory::OrbCollection {
        parent.spawn((
            Text::new("Nothing here yet..."),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgba(0.6, 0.55, 0.7, 0.5)),
        ));
        return;
    }

    for item in items {
        let owned = tracker.purchased.contains(&item.id);
        let affordable = progress.focus_points >= item.cost;
        let is_orb = item.category == ShopCategory::OrbCollection;

        // Item row
        parent
            .spawn(Node {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.0)),
                column_gap: Val::Px(12.0),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            })
            .with_children(|row| {
                // Info column
                row.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(2.0),
                    flex_grow: 1.0,
                    ..default()
                })
                .with_children(|info| {
                    let name_color = if owned {
                        Color::srgba(0.6, 0.55, 0.7, 0.5)
                    } else {
                        Color::srgb(0.9, 0.88, 0.8)
                    };
                    info.spawn((
                        Text::new(item.name),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(name_color),
                    ));
                    info.spawn((
                        Text::new(item.description),
                        TextFont {
                            font_size: 13.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.6, 0.55, 0.7, 0.7)),
                    ));
                });

                // For owned orb items: show Equip/Equipped button
                if is_orb && owned {
                    let orb_type = shop_item_to_orb_type(item.id).unwrap();
                    let is_equipped = equipped.0 == orb_type;
                    let (btn_bg, btn_text_color, btn_label) = if is_equipped {
                        (
                            Color::srgba(0.2, 0.5, 0.25, 0.6),
                            Color::srgb(0.4, 0.9, 0.5),
                            "Equipped",
                        )
                    } else {
                        (
                            Color::srgba(0.3, 0.6, 0.8, 0.8),
                            Color::srgb(0.9, 0.95, 1.0),
                            "Equip",
                        )
                    };

                    row.spawn((
                        Button,
                        Node {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(6.0)),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            justify_content: JustifyContent::Center,
                            min_width: Val::Px(80.0),
                            ..default()
                        },
                        BackgroundColor(btn_bg),
                        EquipButton(orb_type),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(btn_label),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(btn_text_color),
                        ));
                    });
                } else {
                    // Standard Buy button / Owned badge
                    let (btn_bg, btn_text_color, btn_label) = if owned {
                        (
                            Color::srgba(0.2, 0.5, 0.25, 0.6),
                            Color::srgb(0.4, 0.9, 0.5),
                            "Owned".to_string(),
                        )
                    } else if affordable {
                        (
                            Color::srgba(1.0, 0.85, 0.4, 0.9),
                            Color::srgb(0.08, 0.06, 0.14),
                            format!("{} AFP", item.cost),
                        )
                    } else {
                        (
                            Color::srgba(0.3, 0.25, 0.4, 0.5),
                            Color::srgba(0.5, 0.45, 0.6, 0.5),
                            format!("{} AFP", item.cost),
                        )
                    };

                    row.spawn((
                        Button,
                        Node {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(6.0)),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            justify_content: JustifyContent::Center,
                            min_width: Val::Px(80.0),
                            ..default()
                        },
                        BackgroundColor(btn_bg),
                        BuyButton(item.id),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(btn_label),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(btn_text_color),
                        ));
                    });
                }
            });
    }
}

fn spawn_orb_row(
    parent: &mut ChildSpawnerCommands,
    name: &str,
    description: &str,
    orb_type: OrbType,
    is_equipped: bool,
) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(8.0)),
            column_gap: Val::Px(12.0),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            ..default()
        })
        .with_children(|row| {
            row.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(2.0),
                flex_grow: 1.0,
                ..default()
            })
            .with_children(|info| {
                info.spawn((
                    Text::new(name),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.88, 0.8)),
                ));
                info.spawn((
                    Text::new(description),
                    TextFont {
                        font_size: 13.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.6, 0.55, 0.7, 0.7)),
                ));
            });

            let (btn_bg, btn_text_color, btn_label) = if is_equipped {
                (
                    Color::srgba(0.2, 0.5, 0.25, 0.6),
                    Color::srgb(0.4, 0.9, 0.5),
                    "Equipped",
                )
            } else {
                (
                    Color::srgba(0.3, 0.6, 0.8, 0.8),
                    Color::srgb(0.9, 0.95, 1.0),
                    "Equip",
                )
            };

            row.spawn((
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(6.0)),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    justify_content: JustifyContent::Center,
                    min_width: Val::Px(80.0),
                    ..default()
                },
                BackgroundColor(btn_bg),
                EquipButton(orb_type),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new(btn_label),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(btn_text_color),
                ));
            });
        });
}

fn spawn_generator_items(
    parent: &mut ChildSpawnerCommands,
    generators: &GeneratorState,
    synergies: &SynergyState,
    progress: &ArcaneProgress,
    transcendence: &TranscendenceState,
    resources: &SecondaryResources,
) {
    let mut any_visible = false;
    let discount = transcendence.generator_cost_discount();

    for gt in GeneratorType::ALL {
        if progress.total_truths < gt.unlock_threshold() {
            continue;
        }
        any_visible = true;

        let owned = generators.count(gt);
        let cost = gt.next_cost_discounted(owned, discount);
        let serenity_cost = gt.serenity_cost();
        let has_serenity = serenity_cost.map_or(true, |s| resources.serenity >= s);
        let affordable = progress.focus_points >= cost && has_serenity;
        let production = gt.base_production();
        let syn_mult = synergies.total_mult(gt);

        parent
            .spawn(Node {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.0)),
                column_gap: Val::Px(12.0),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            })
            .with_children(|row| {
                // Info column
                row.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(2.0),
                    flex_grow: 1.0,
                    ..default()
                })
                .with_children(|info| {
                    // Name + owned count
                    let name_label = if owned > 0 {
                        format!("{} ({})", gt.name(), owned)
                    } else {
                        gt.name().to_string()
                    };
                    info.spawn((
                        Text::new(name_label),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.88, 0.8)),
                    ));

                    // Description + production info
                    let effective_per_unit = production * syn_mult;
                    let desc = if owned > 0 {
                        if syn_mult > 1.001 {
                            format!(
                                "{} (+{:.1}/s each x{:.2}, {:.1}/s total)",
                                gt.description(),
                                production,
                                syn_mult,
                                effective_per_unit * owned as f64,
                            )
                        } else {
                            format!(
                                "{} (+{:.1}/s each, {:.1}/s total)",
                                gt.description(),
                                production,
                                gt.production(owned),
                            )
                        }
                    } else if syn_mult > 1.001 {
                        format!(
                            "{} (+{:.1} wisdom/s x{:.2})",
                            gt.description(),
                            production,
                            syn_mult
                        )
                    } else {
                        format!("{} (+{:.1} wisdom/s)", gt.description(), production)
                    };
                    info.spawn((
                        Text::new(desc),
                        TextFont {
                            font_size: 13.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.6, 0.55, 0.7, 0.7)),
                    ));

                    // Serenity cost line
                    if let Some(s_cost) = serenity_cost {
                        let color = if has_serenity {
                            Color::srgba(0.4, 0.7, 0.9, 0.8)
                        } else {
                            Color::srgba(0.9, 0.4, 0.3, 0.8)
                        };
                        info.spawn((
                            Text::new(format!("Requires {:.0} Serenity", s_cost)),
                            TextFont {
                                font_size: 11.0,
                                ..default()
                            },
                            TextColor(color),
                        ));
                    }

                    // Synergy details line
                    if let Some(syn_desc) = synergies.synergy_description(gt, generators) {
                        info.spawn((
                            Text::new(syn_desc),
                            TextFont {
                                font_size: 11.0,
                                ..default()
                            },
                            TextColor(Color::srgba(0.5, 0.8, 0.6, 0.7)),
                        ));
                    }
                });

                // Buy button
                let (btn_bg, btn_text_color) = if affordable {
                    (
                        Color::srgba(1.0, 0.85, 0.4, 0.9),
                        Color::srgb(0.08, 0.06, 0.14),
                    )
                } else {
                    (
                        Color::srgba(0.3, 0.25, 0.4, 0.5),
                        Color::srgba(0.5, 0.45, 0.6, 0.5),
                    )
                };

                row.spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(14.0), Val::Px(6.0)),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        justify_content: JustifyContent::Center,
                        min_width: Val::Px(90.0),
                        ..default()
                    },
                    BackgroundColor(btn_bg),
                    BuyGeneratorButton(gt),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(format_afp(cost)),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(btn_text_color),
                    ));
                });
            });
    }

    if !any_visible {
        parent.spawn((
            Text::new("Generate more truths to unlock generators..."),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgba(0.6, 0.55, 0.7, 0.5)),
        ));
    }
}

pub fn handle_buy_generator(
    interactions: Query<(&Interaction, &BuyGeneratorButton), Changed<Interaction>>,
    mut generators: ResMut<GeneratorState>,
    mut progress: ResMut<ArcaneProgress>,
    mut resources: ResMut<SecondaryResources>,
    transcendence: Res<TranscendenceState>,
) {
    let discount = transcendence.generator_cost_discount();
    for (interaction, button) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let owned = generators.count(button.0);
        let cost = button.0.next_cost_discounted(owned, discount);

        if progress.focus_points < cost {
            continue;
        }

        // Check serenity requirement for high-tier generators
        if let Some(serenity_cost) = button.0.serenity_cost() {
            if resources.serenity < serenity_cost {
                continue;
            }
            resources.serenity -= serenity_cost;
        }

        progress.focus_points -= cost;
        generators.add(button.0);
    }
}

/// Format large AFP values with K/M/B suffixes for readability
fn format_afp(value: u64) -> String {
    if value >= 1_000_000_000 {
        format!("{:.1}B AFP", value as f64 / 1_000_000_000.0)
    } else if value >= 1_000_000 {
        format!("{:.1}M AFP", value as f64 / 1_000_000.0)
    } else if value >= 10_000 {
        format!("{:.1}K AFP", value as f64 / 1_000.0)
    } else {
        format!("{} AFP", value)
    }
}
