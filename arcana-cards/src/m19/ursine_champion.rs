//! Ursine Champion — `{1}{G}` 2/2 green Human Berserker.
//! "{5}{G}: This creature gets +3/+3 and becomes a Bear Berserker until end
//! of turn. Activate only once each turn."
//!
//! GAP: "becomes a Bear Berserker" — subtype change not expressible.
//! "Activate only once each turn" enforced via `once_per_turn`.
//! The +3/+3 pump is expressible.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ursine Champion");
    let human = reg.interner_mut().intern("Human");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(berserker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{G}: This creature gets +3/+3 and becomes a Bear Berserker until end of turn. Activate only once each turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{G}").unwrap(),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_bear,
            }),
    )
}

fn become_bear(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a Bear Berserker" subtype change not expressible
    vec![Effect::Pump {
        target: ctx.source,
        power: 3,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
