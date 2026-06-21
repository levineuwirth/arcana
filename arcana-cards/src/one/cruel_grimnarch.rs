//! Cruel Grimnarch — `{5}{B}` 5/5 Phyrexian Cleric.
//!
//! Oracle:
//! * Deathtouch.
//! * "When this creature enters, each opponent discards a card. For each
//!   opponent who can't, you gain 4 life." — ETB trigger. Evaluated at
//!   resolution: each opponent with at least one card discards one; each
//!   opponent with an empty hand (who "can't") yields 4 life. Fully
//!   expressible via per-opponent Sequence.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cruel Grimnarch");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_discard_or_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_discard_or_gain(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        if script::hand_size(state, opp) == 0 {
            // The opponent "can't" discard → you gain 4 life.
            effects.push(Effect::GainLife {
                player: trig.controller,
                amount: 4,
            });
        } else {
            effects.push(Effect::Discard {
                player: opp,
                count: 1,
                choice: DiscardChoice::ControllerChooses,
            });
        }
    }
    if effects.is_empty() {
        Vec::new()
    } else {
        vec![Effect::Sequence(effects)]
    }
}
