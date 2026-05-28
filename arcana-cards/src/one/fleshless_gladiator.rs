//! Fleshless Gladiator — `{1}{B}` 2/2 Phyrexian Skeleton.
//! Corrupted — `{2}{B}: Return this card from your graveyard to the battlefield tapped.
//! You lose 1 life. Activate only if an opponent has three or more poison counters.`
//! GAP: ActivationZone::Graveyard not in catalog. GAP: "opponent has 3+ poison counters"
//! precondition not checkable.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::Effect;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fleshless Gladiator");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(skeleton);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Corrupted — {2}{B}: Return this card from your graveyard to the battlefield tapped. You lose 1 life. Activate only if opponent has 3+ poison counters.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: corrupted_reanimate,
            }),
    )
}

fn corrupted_reanimate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: ActivationZone::Graveyard not in catalog.
    // GAP: "return from graveyard to battlefield tapped" — no self-reanimate Effect.
    // GAP: "only if opponent has 3+ poison counters" — no poison counter check.
    Vec::new()
}
