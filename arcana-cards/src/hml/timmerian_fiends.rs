//! Timmerian Fiends — `{1}{B}{B}` 1/1 Horror.
//! "Remove this card from your deck before playing if you're not
//! playing for ante."
//! "{B}{B}{B}, Sacrifice this creature: The owner of target artifact
//! may ante the top card of their library. If that player doesn't,
//! exchange ownership of that artifact and Timmerian Fiends. ..."
//!
//! No keywords. The activated ability's cost ({B}{B}{B} + sacrifice
//! self) and its target (an artifact) are wired, but its effect is the
//! ante / ownership-exchange mechanic which has no expressible
//! primitive — GAP'd. The deck-construction ante caveat is a static
//! game-rule note with no in-game effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Timmerian Fiends");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}{B}{B}, Sacrifice this creature: The owner of target artifact may ante the top card of their library. If that player doesn't, exchange ownership of that artifact and Timmerian Fiends."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}{B}{B}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ante_exchange,
            }),
    )
}

fn ante_exchange(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: ante / permanent ownership-exchange mechanic — no expressible
    // ante or change-of-ownership Effect primitive.
    Vec::new()
}
