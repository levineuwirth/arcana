//! Sinew Dancer — `{W}` 1/1 white Phyrexian Soldier.
//!
//! Oracle:
//! * `{3}{W}, {T}: Tap target creature.`
//! * Corrupted — `{W}, {T}: Tap target creature. Activate only if an opponent
//!   has three or more poison counters.`
//!
//! Both activated abilities are wired (mana + tap cost, targeting a creature,
//! resolving to `Effect::Tap`). The "Corrupted" keyword Scryfall reports is just
//! ability-word flavor for the cheaper variant — there is no `KeywordAbility`
//! variant for it, so `keywords` stays empty. The "Activate only if an opponent
//! has three or more poison counters" precondition has no demonstrated
//! `activation_condition` helper for opponent poison counts, so that gate is
//! GAP'd (the ability is otherwise faithfully emitted).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sinew Dancer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // "Corrupted" is an ability word, not an expressible KeywordAbility.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{W}, {T}: Tap target creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_target_creature,
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "Activate only if an opponent has three or more poison
                // counters" — no demonstrated activation_condition helper for an
                // opponent's poison-counter count; the gate is omitted.
                text: "Corrupted — {W}, {T}: Tap target creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_target_creature,
            }),
    )
}

fn tap_target_creature(
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
    vec![Effect::Tap { target: *id }]
}
