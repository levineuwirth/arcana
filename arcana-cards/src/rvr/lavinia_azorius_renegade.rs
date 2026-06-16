//! Lavinia, Azorius Renegade — `{W}{U}` 2/2 Legendary Human Soldier.
//! "Each opponent can't cast noncreature spells with mana value greater
//! than the number of lands that player controls."
//! "Whenever an opponent casts a spell, if no mana was spent to cast it,
//! counter that spell."
//!
//! The first line is a casting-restriction static (no Effect for it).
//! The second is an opponent-cast trigger; its "if no mana was spent"
//! intervening-if is not expressible with the available `conditions::`
//! predicates, and there is no accessor to recover the triggering
//! spell's stack id for `Effect::Counter`, so the effect body is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lavinia, Azorius Renegade");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static — "each opponent can't cast noncreature spells with
    // mana value greater than the lands they control"; no casting
    // restriction Effect available.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: counter_free_spell,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn counter_free_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if no mana was spent to cast it, counter that spell" — the
    // intervening-if (no mana spent) is not expressible, and there is no
    // accessor for the triggering spell's stack id to feed Effect::Counter.
    Vec::new()
}
