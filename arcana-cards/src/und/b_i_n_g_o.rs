//! B-I-N-G-O — `{1}{G}` 1/1 Dog with Trample.
//!
//! Oracle:
//! * Trample.
//! * Whenever a player casts a spell, put a chip counter on its mana value.
//! * This creature gets +9/+9 for each set of three numbers in a row with
//!   chip counters on them.
//!
//! The Trample keyword is a base characteristic. The spell-cast trigger's
//! effect ("put a chip counter on its mana value" — a bingo-board state that
//! lives nowhere in the engine) and the static +9/+9 sweep keyed off that
//! board are both inexpressible with the demonstrated API.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("B-I-N-G-O");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Whenever a player casts a spell, put a chip counter on its mana
            // value." The trigger fires correctly, but its payload puts a
            // counter onto an abstract bingo board (the mana-value cells), which
            // is not a game object — there is no Effect that models it.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: bingo_chip,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: static "gets +9/+9 for each set of three numbers in a row with chip
    // counters" — depends on the unmodeled bingo-board state; no Effect for it.
}

fn bingo_chip(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "put a chip counter on its mana value" tracks marks on a 1..=N bingo
    // board, not on any GameObject; no Effect / counter target expresses it.
    Vec::new()
}
