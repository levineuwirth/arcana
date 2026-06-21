//! Tetsuo Umezawa — `{U}{B}{R}` 3/3 Legendary Human Archer.
//!
//! Oracle:
//! * "Tetsuo Umezawa can't be the target of Aura spells." — a static
//!   targeting-restriction; not expressible as a triggered/activated
//!   ability, GAP'd.
//! * "{U}{B}{B}{R}, {T}: Destroy target tapped or blocking creature."
//!   — a mana+tap activated removal ability. The "tapped OR blocking"
//!   target restriction is a disjunction a single ObjectFilter cannot
//!   express, so the target filter is the looser "creature" and the
//!   exact restriction is a documented fidelity gap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tetsuo Umezawa");
    let human = reg.interner_mut().intern("Human");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "can't be the target of Aura spells" — a continuous
    // targeting restriction, not a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}{B}{B}{R}, {T}: Destroy target tapped or blocking creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}{B}{B}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                // GAP refinement: "tapped or blocking" disjunction not
                // expressible in one ObjectFilter; targets any creature.
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_target_creature,
            }),
    )
}

fn destroy_target_creature(
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
    vec![Effect::DestroyPermanent { target: *id }]
}
