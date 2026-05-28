//! Zombie Assassin — `{4}{B}` 3/2 Zombie Assassin.
//! `{T}, Exile two cards from your graveyard and this creature: Destroy target nonblack creature. It can't be regenerated.`
//! Note: "exile this creature" is modeled with sacrifice: true (self-exile approximation).
//! GAP: ActivationCost has no field for "exile two cards from your graveyard";
//! also "it can't be regenerated" is not a flag we can set on DestroyPermanent.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zombie Assassin");
    let zombie = reg.interner_mut().intern("Zombie");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(assassin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Exile two cards from your graveyard and this creature: Destroy target nonblack creature. It can't be regenerated.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true, // approximation for "exile this creature"
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().without_colors(ColorSet::black()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_nonblack,
            }),
    )
}

fn destroy_nonblack(
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
    // GAP: ActivationCost has no "exile two cards from graveyard" field.
    // GAP: DestroyPermanent has no "can't be regenerated" flag.
    vec![Effect::DestroyPermanent { target: *id }]
}
