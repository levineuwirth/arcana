//! Thunderclap Drake — `{1}{U}` 2/1 blue Drake with Flying.
//!
//! Oracle text:
//! * Flying.
//! * Instant and sorcery spells you cast cost {1} less to cast.
//! * {2}{U}, Sacrifice this creature: When you next cast an instant or
//!   sorcery spell this turn, copy it for each time you've cast your
//!   commander from the command zone this game. You may choose new
//!   targets for the copies.
//!
//! Implemented: the Flying keyword and the {2}{U}, Sacrifice activated
//! ability is wired with its correct cost.
//!
//! GAP: the "instant and sorcery spells you cast cost {1} less" static
//! cost reduction has no `Effect`/static representation — omitted.
//! GAP: the activated ability's effect is a delayed "when you next cast
//! …, copy it N times where N = times you've cast your commander" rider
//! — neither the delayed next-cast copy count nor the commander-cast
//! tally is expressible, so the resolver returns no effects.

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
    let name = reg.interner_mut().intern("Thunderclap Drake");
    let drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{U}, Sacrifice this creature: When you next cast an instant or sorcery spell this turn, copy it for each time you've cast your commander from the command zone this game.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{U}").expect("valid cost"),
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: delayed_copy,
        }),
    )
}

fn delayed_copy(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: delayed "when you next cast an instant/sorcery, copy it for
    // each time you've cast your commander" — no expressible primitive.
    Vec::new()
}
