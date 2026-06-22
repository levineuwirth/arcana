//! Zellix, Sanity Flayer — `{2}{U}` 2/3 Legendary Horror.
//!
//! Hive Mind — Whenever a player mills one or more creature cards, you create
//!   a 1/1 black Horror creature token. (GAP: no mill TriggerCondition)
//! {1}, {T}: Target player mills three cards.
//! Choose a Background (GAP: commander partner marker)
//!
//! The "Hive Mind" / "Choose a Background" lines are not expressible — there
//! is no "a player mills creature cards" trigger condition, and Background
//! partnering is a commander-format marker. The {1}, {T} targeted mill
//! activated ability IS wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zellix, Sanity Flayer");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "Choose a Background" — commander partner marker, no keyword.
        ..Default::default()
    };

    // GAP: "Hive Mind — Whenever a player mills one or more creature cards,
    // you create a 1/1 black Horror creature token." — no mill trigger.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Target player mills three cards.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: mill_target_player,
            }),
    )
}

fn mill_target_player(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::Mill { player: *p, count: 3 }]
}
