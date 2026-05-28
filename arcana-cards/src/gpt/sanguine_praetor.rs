//! Sanguine Praetor — `{6}{B}{B}` 7/5 Avatar Praetor.
//! `{B}, Sacrifice a creature: Destroy each creature with the same mana value as the sacrificed creature.`
//! GAP: "sacrifice a creature" (not self) as cost — using self-sacrifice as approximation.
//! GAP: "destroy each creature with the same mana value as the sacrificed creature" — dynamic CMC-matching destroy not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sanguine Praetor");
    let avatar = reg.interner_mut().intern("Avatar");
    let praetor = reg.interner_mut().intern("Praetor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    subtypes.0.insert(praetor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, Sacrifice a creature: Destroy each creature with the same mana value as the sacrificed creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").unwrap(),
                    sacrifice: true, // GAP: should sacrifice another creature, not self
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_same_cmc,
            }),
    )
}

fn destroy_same_cmc(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "destroy each creature with the same mana value as the sacrificed creature" — dynamic CMC-matching not expressible
    Vec::new()
}
