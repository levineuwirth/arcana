//! Ancient Cellarspawn — `{1}{B}{B}` 3/3 Enchantment Creature — Horror.
//!
//! * "Each spell you cast that's a Demon, Horror, or Nightmare costs {1} less."
//!   GAP: static cost-reduction is not expressible with the documented
//!   triggered/activated/keyword surface.
//! * "Whenever you cast a spell, if the amount of mana spent to cast it was
//!   less than its mana value, target opponent loses life equal to the
//!   difference." GAP: the mana-spent-vs-mana-value delta is not available to
//!   an effect fn; emit the trigger structure with the body GAP'd.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Ancient Cellarspawn");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    // GAP: "Each spell you cast that's a Demon, Horror, or Nightmare costs {1}
    // less to cast." — static cost reduction is not in the documented surface.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cellarspawn_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

// GAP: "if mana spent < mana value, target opponent loses life equal to the
// difference" — neither the mana actually spent nor the resulting delta is
// exposed to an effect fn, and the intervening-if predicate needs that delta.
fn cellarspawn_drain(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
