//! Zerapa Minotaur — `{2}{R}{R}` 3/3 red Minotaur with First strike.
//!
//! Oracle:
//! * First strike (keyword).
//! * `{2}: This creature loses first strike until end of turn. Any player may
//!   activate this ability.`
//!
//! The activated ability's `{2}` cost is modeled, but the effect is GAP'd:
//! there is no effect that removes a SINGLE named keyword (`LoseAllAbilities`
//! strips everything, which would be materially wrong), and "any player may
//! activate" is not an expressible activation modifier.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zerapa Minotaur");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: This creature loses first strike until end of turn. Any player may activate this ability.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: lose_first_strike,
            }),
    )
}

fn lose_first_strike(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no single-keyword-removal effect (LoseAllAbilities overshoots);
    // "any player may activate" is also not expressible.
    Vec::new()
}
