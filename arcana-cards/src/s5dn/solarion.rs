//! Solarion — `{7}` 0/0 Artifact Creature — Construct.
//!
//! Oracle:
//! * Sunburst (enters with a +1/+1 counter for each color of mana spent
//!   to cast it) — the `KeywordAbility::Sunburst` keyword is supported.
//! * "{T}: Double the number of +1/+1 counters on this creature." — a tap
//!   activation. The cost (tap) is expressible, but there is no
//!   "double counters" / multiply-by-N Effect primitive, so the ability's
//!   effect body is GAP'd (no AddCounters amount can express "double").

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Solarion");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Sunburst],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Double the number of +1/+1 counters on this creature.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: double_counters,
            }),
    )
}

fn double_counters(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no "double the number of +1/+1 counters" Effect primitive
    // (AddCounters takes a fixed count; there is no multiply-by-current).
    Vec::new()
}
