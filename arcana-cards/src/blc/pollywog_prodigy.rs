//! Pollywog Prodigy — `{1}{U}` 1/3 blue Frog Wizard with Evolve.
//! "Whenever an opponent casts a noncreature spell with mana value less
//! than this creature's power, draw a card."
//!
//! Evolve is a base keyword. The cast trigger is wired as an opponent-
//! cast of a noncreature spell; the "with mana value less than this
//! creature's power" gate is dynamic (depends on the source's current
//! power) and there is no spell-mana-value predicate available, so that
//! restriction is GAP'd and the trigger fires on any opponent noncreature
//! spell.

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
    let name = reg.interner_mut().intern("Pollywog Prodigy");
    let frog = reg.interner_mut().intern("Frog");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Evolve],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Whenever an opponent casts a noncreature spell with mana
            // value less than this creature's power, draw a card."
            // GAP: the "mana value less than this creature's power" gate
            // is dynamic and not expressible (no spell-mv filter), so the
            // filter only restricts to noncreature opponent spells.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: draw_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_a_card(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
