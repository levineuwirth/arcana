//! Veko, Death's Doorkeeper — `{W}{B}` 1/3 Legendary Spirit Cleric.
//!
//! Extort
//! {T}, Sacrifice a non-Spirit creature: Return target creature card from
//! your graveyard to your hand. It perpetually becomes a Spirit, has base
//! power and toughness 1/1, and gains "You may pay {W/B} rather than pay
//! this spell's mana cost." Activate only as a sorcery.
//!
//! Decomposed as: Extort (GAP'd — not in the usable keyword surface) plus
//! one activated ability. The cost is {T} + Sacrifice a non-Spirit creature
//! (an enumerated `sacrifice_other`), and the ability returns the target
//! creature card from your graveyard to your hand. The perpetual riders
//! (becomes a Spirit / base 1/1 / gains an alternative cost) are perpetual
//! modifications not expressible by any Effect, so they are GAP'd.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Veko, Death's Doorkeeper");
    let spirit = reg.interner_mut().intern("Spirit");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(cleric);

    // GAP: keyword "Extort" is not in the usable keyword surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let non_spirit = ObjectFilter::creature().without_subtype_sym(spirit);

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice a non-Spirit creature: Return target creature card from your graveyard to your hand. It perpetually becomes a Spirit, has base power and toughness 1/1, and gains \"You may pay {W/B} rather than pay this spell's mana cost.\" Activate only as a sorcery.".into(),
            cost: ActivationCost {
                tap: true,
                sacrifice_other: Some(non_spirit),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: return_creature_card,
        }),
    )
}

fn return_creature_card(
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
    // GAP: "It perpetually becomes a Spirit, has base power and toughness
    // 1/1, and gains an alternative {W/B} cost" — perpetual modifications
    // are not expressible by any Effect variant.
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
