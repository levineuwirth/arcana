//! Scion of Shiv — `{2}{R}{R}` 3/3 Dragon with Flying.
//!
//! Oracle:
//! * Flying.
//! * `{2}{R}: Scion of Shiv perpetually gets +1/+0.`
//!
//! "Perpetually" (Alchemy) persists across zones; the closest engine
//! expression is a battlefield-permanent pump (`Duration::Permanent`),
//! which is faithful while the creature stays on the battlefield.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scion of Shiv");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{R}: Scion of Shiv perpetually gets +1/+0.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: perpetual_pump,
        }),
    )
}

fn perpetual_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Fidelity note: "perpetually" survives leaving/re-entering the
    // battlefield; Duration::Permanent only persists on the battlefield.
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::Permanent,
        keywords: vec![],
    }]
}
