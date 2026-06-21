//! Dan, Shrewd Trader — `{3}{B}` 3/4 Legendary Creature — Human Gamer.
//!
//! Oracle:
//! * "You can bargain any permanent instead of just artifacts, enchantments,
//!   and tokens." — a static rule-modifying ability altering what may be
//!   sacrificed to the bargain additional cost. Not expressible with the
//!   demonstrated API (no bargain-cost machinery). GAP'd.
//! * "Whenever you cast a spell, if it was bargained, copy that spell. You may
//!   choose new targets for the copy." — the SpellCast trigger is expressible,
//!   but the "if it was bargained" intervening condition has no predicate in the
//!   available `conditions::` set, so the gate can't be honored. Emitting the
//!   copy unconditionally would be materially wrong, so the trigger effect is
//!   GAP'd to a no-op.

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
    let name = reg.interner_mut().intern("Dan, Shrewd Trader");
    let human = reg.interner_mut().intern("Human");
    let gamer = reg.interner_mut().intern("Gamer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(gamer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "You can bargain any permanent instead of just artifacts,
    // enchantments, and tokens" — alters the bargain additional cost; no
    // bargain machinery in the demonstrated API.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: copy_if_bargained,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn copy_if_bargained(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it was bargained, copy that spell" — no predicate to detect
    // whether the just-cast spell was bargained, so the conditional copy can't
    // be gated; firing it unconditionally would be wrong.
    Vec::new()
}
