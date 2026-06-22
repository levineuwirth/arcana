//! Wildborn Preserver — `{1}{G}` 2/2 Creature — Elf Archer.
//! Flash, Reach.
//! Whenever another non-Human creature you control enters, you may pay {X}.
//! When you do, put X +1/+1 counters on this creature.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wildborn Preserver");
    let elf = reg.interner_mut().intern("Elf");
    let archer = reg.interner_mut().intern("Archer");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Reach],
        ..Default::default()
    };

    let non_human_creature = ObjectFilter::creature()
        .without_subtype_sym(human)
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // "Whenever another non-Human creature you control enters."
            trigger_condition: TriggerCondition::ZoneChange {
                filter: non_human_creature,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: pay_x_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn pay_x_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {X}. When you do, put X +1/+1 counters on this
    // creature." OptionalPaymentKind supports only a FIXED Mana cost or a Life
    // amount — there is no variable-{X} mana payment, and X then feeds the
    // counter count. The pay-X / put-X reflexive coupling is not expressible,
    // so the whole effect is GAP'd. (The "another" self-exclusion on the
    // trigger filter is also approximate.)
    Vec::new()
}
