//! Colossus of Sardia — `{9}` 9/9 Artifact Creature — Golem with Trample.
//!
//! Oracle:
//! * Trample
//! * This creature doesn't untap during your untap step. (static — GAP)
//! * `{9}: Untap this creature. Activate only during your upkeep.`
//!
//! Trample is a base keyword. The "doesn't untap during your untap step"
//! static has no expressible primitive and is GAP'd. The `{9}: Untap` ability
//! is emitted as a self-untap activated ability; the "only during your upkeep"
//! timing window has no expressible constructor (no activation_condition
//! builder for an upkeep restriction), so the timing gate is GAP'd while the
//! untap effect itself is faithful.

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
    let name = reg.interner_mut().intern("Colossus of Sardia");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{9}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: static — "This creature doesn't untap during your untap step."
    // No expressible don't-untap restriction primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: timing — "Activate only during your upkeep" cannot be
                // restricted with the available constructors.
                text: "{9}: Untap this creature. Activate only during your upkeep.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{9}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_self,
            }),
    )
}

fn untap_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}
