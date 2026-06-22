//! Romana II — `{3}{W}` 3/3 Legendary Time Lord Scientist with Vigilance.
//! "{1}, {T}: Create a tapped token that's a copy of target token that entered
//!  this turn. Doctor's companion."
//!
//! Vigilance is a base keyword. The activated ability mints a token copy of a
//! target token via CopyPermanent. Fidelity gaps: the copy can't be forced
//! "tapped" (CopyPermanent has no tapped flag), and "that entered this turn"
//! can't be filtered (no entered-this-turn ObjectFilter predicate) — the target
//! is restricted to tokens only as the closest expressible filter. Doctor's
//! companion is a commander-format static keyword with no rules effect — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Romana II");
    let time_lord = reg.interner_mut().intern("Time Lord");
    let scientist = reg.interner_mut().intern("Scientist");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(time_lord);
    subtypes.0.insert(scientist);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: "Doctor's companion" — commander deck-construction static, no rules
    // effect and not a usable KeywordAbility.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}, {T}: Create a tapped token that's a copy of target token \
                   that entered this turn."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::permanent().tokens_only()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: copy_token,
        }),
    )
}

fn copy_token(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // Fidelity gap: the new copy can't be forced tapped (no tapped flag on
    // CopyPermanent).
    vec![Effect::CopyPermanent { target: *id }]
}
