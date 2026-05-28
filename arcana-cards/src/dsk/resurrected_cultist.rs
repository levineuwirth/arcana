//! Resurrected Cultist — `{2}{B}` 4/1 black Human Cleric. "Delirium —
//! {2}{B}{B}: Return this card from your graveyard to the battlefield with
//! a finality counter on it. Activate only if there are four or more card
//! types among cards in your graveyard and only as a sorcery."
//!
//! GAP: "finality counter" CounterKind not available; "four or more card types
//! in graveyard" precondition not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Resurrected Cultist");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}{B}: Return from graveyard if delirium. (GAP: finality counter + delirium check)".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}{B}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: delirium_reanimate,
            }),
    )
}

fn delirium_reanimate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "finality counter" CounterKind not available; delirium check not expressible.
    // Emitting simple reanimate without counter.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
