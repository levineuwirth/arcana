//! Ral's Staticaster — `{2}{U}{R}` 3/3 Creature — Lizard Wizard.
//!
//! Oracle:
//! * Trample — keyword.
//! * "Whenever this creature attacks, if you control a Ral planeswalker,
//!   this creature gets +1/+0 for each card in your hand until end of
//!   turn." — SelfAttacks trigger gated by an intervening-if ("you control
//!   a Ral planeswalker" = a planeswalker with the subtype Ral); the
//!   effect pumps +X/+0 where X = cards in your hand.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ral's Staticaster");
    let lizard = reg.interner_mut().intern("Lizard");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: Some(if_control_ral_planeswalker),
                effect: pump_per_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_control_ral_planeswalker(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    // "a Ral planeswalker" = a planeswalker with the subtype Ral.
    let filter = script::subtype_filter(reg, "Ral").with_types(TypeLine::PLANESWALKER.into());
    conditions::you_control_a(s, you, &filter)
}

fn pump_per_hand(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::hand_size(state, trig.controller) as i32;
    vec![Effect::Pump {
        target: trig.source,
        power: n,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
