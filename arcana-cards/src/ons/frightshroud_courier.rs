//! Frightshroud Courier — `{2}{B}` 2/1 black Zombie.
//!
//! Oracle text:
//! * "You may choose not to untap this creature during your untap step."
//!   A static untap-restriction (a may-skip-untap permission). The demonstrated
//!   API exposes no untap-restriction static / replacement primitive, so this
//!   line is GAP'd — see the doc note below; nothing is wired for it.
//! * "{2}{B}, {T}: Target Zombie creature gets +2/+2 and has fear for as long
//!   as this creature remains tapped." — wired as an activated ability: a
//!   mana + tap cost granting +2/+2 and Fear to a target Zombie creature.
//!   FIDELITY GAP: the duration "for as long as this creature remains tapped"
//!   is not an expressible Duration in the demonstrated API; the closest
//!   available is `Duration::EndOfTurn`, which is used here (the boost lasts
//!   until end of turn rather than tracking this creature's tapped status).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frightshroud Courier");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP (static): "You may choose not to untap this creature during your untap
    // step." — no untap-restriction static / replacement primitive is available
    // in the demonstrated API. Not wired.

    let zombie_sub = reg.interner_mut().intern("Zombie");

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{B}, {T}: Target Zombie creature gets +2/+2 and has fear for as long as this creature remains tapped.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new()
                        .with_types(TypeLine::CREATURE.into())
                        .with_subtypes_any(vec![zombie_sub]),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_zombie_fear,
        }),
    )
}

/// `{2}{B}, {T}: Target Zombie creature gets +2/+2 and has fear …`
///
/// FIDELITY GAP: granted until end of turn rather than "for as long as this
/// creature remains tapped" (no such Duration is available).
fn pump_zombie_fear(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Fear],
    }]
}
