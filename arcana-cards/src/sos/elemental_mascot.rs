//! Elemental Mascot — `{1}{U}{R}` 1/4 Elemental Bird with Flying and Vigilance.
//! "Opus — Whenever you cast an instant or sorcery spell, this creature gets
//!  +1/+0 until end of turn. If five or more mana was spent to cast that spell,
//!  exile the top card of your library. You may play that card until the end of
//!  your next turn."
//!
//! Flying and Vigilance are expressible (Opus is an ability-word label, not a
//! usable KeywordAbility variant). The trigger fires when you cast an instant or
//! sorcery spell and pumps this creature +1/+0 until end of turn. The "if five
//! or more mana was spent … exile the top card, play until end of your next
//! turn" rider has no accessor for mana spent on the casting — GAP'd; the +1/+0
//! is wired (partial).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Elemental Mascot");
    let elemental = reg.interner_mut().intern("Elemental");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(bird);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                ),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: cast_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn cast_pump(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "If five or more mana was spent to cast that spell, exile the top card
    // of your library; you may play it until end of your next turn." — no
    // accessor for mana spent on the casting; the +1/+0 pump is wired.
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
