//! Emrakul's Evangel — `{2}{G}` 3/2 green Human Horror.
//! "{T}, Sacrifice this creature and any number of other non-Eldrazi creatures:
//! Create a 3/2 colorless Eldrazi Horror creature token for each creature
//! sacrificed this way."
//!
//! GAP: "sacrifice this and any number of other non-Eldrazi creatures" —
//! ActivationCost only supports self-sacrifice.
//! GAP: "for each creature sacrificed" — count not trackable post-sacrifice.

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
    let name = reg.interner_mut().intern("Emrakul's Evangel");
    let human = reg.interner_mut().intern("Human");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice this and others: Create 3/2 Eldrazi Horror tokens for each sacrificed.".into(),
                // GAP: "sacrifice this and any number of others" (multi-non-self) not expressible.
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: create_eldrazi,
            }),
    )
}

fn create_eldrazi(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "for each creature sacrificed" — count not trackable.
    Vec::new()
}
