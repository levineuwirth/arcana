//! Tenured Concocter — `{4}{G}` 4/5 green Troll Druid. Vigilance.
//! "Whenever this creature becomes the target of a spell or ability an
//!  opponent controls, you may draw a card.
//!  Infusion — This creature gets +2/+0 as long as you gained life this
//!  turn."
//!
//! Keyword line + the becomes-target trigger are expressed.
//! Note: "you may draw a card" is modeled as an unconditional draw (the
//!       optionality is a documented fidelity gap).
//! GAP: "Infusion — gets +2/+0 as long as you gained life this turn" is a
//!      conditional static continuous buff, not a triggered/activated
//!      ability — not expressible in this card class.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Tenured Concocter");
    let troll = reg.interner_mut().intern("Troll");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(troll);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: may_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn may_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may draw a card" — modeled as drawing a card.
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
