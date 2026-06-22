//! Silverquill Silencer — `{W}{B}` 3/2 Human Cleric (B/W).
//!
//! * "As this creature enters, choose a nonland card name." — GAP (no
//!   "choose a card name and remember it" ETB primitive; the catalog's
//!   name mechanic, `NameCardAndExile`, is a one-shot deck-strip, not a
//!   remembered-name store).
//! * "Whenever an opponent casts a spell with the chosen name, they
//!   lose 3 life and you draw a card."
//!
//! GAP (trigger effect): the `SpellCast` trigger condition can filter by
//! object characteristics but NOT by a per-permanent CHOSEN name set at
//! ETB. Firing on every opponent spell (filter: None) and applying the
//! 3-life-loss + draw would be a materially wrong card, so the effect is
//! GAP'd. The opponent-cast trigger is wired structurally only.

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
    let name = reg.interner_mut().intern("Silverquill Silencer");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: chosen_name_punish,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn chosen_name_punish(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "a spell with the chosen name" — no per-permanent remembered
    // card-name store; can't gate the lose-3 / draw on the ETB-chosen
    // name, so the effect is omitted rather than over-firing.
    Vec::new()
}
