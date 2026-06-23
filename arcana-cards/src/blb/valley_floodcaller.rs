//! Valley Floodcaller — `{2}{U}` 2/2 Otter Wizard (U).
//!
//! * Flash (keyword).
//! * "You may cast noncreature spells as though they had flash."
//!   GAP: a static permission-granting ability (no trigger / cost); no
//!   demonstrated primitive grants flash to spells in hand.
//! * "Whenever you cast a noncreature spell, Birds, Frogs, Otters, and Rats you
//!   control get +1/+1 until end of turn. Untap them." Wired as a SpellCast
//!   trigger; the resolver enumerates the matching tribal creatures and pumps +
//!   untaps each.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Valley Floodcaller");
    let otter = reg.interner_mut().intern("Otter");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(otter);
    subtypes.0.insert(wizard);
    // Pre-intern the tribal subtypes so the resolver can look them up.
    reg.interner_mut().intern("Bird");
    reg.interner_mut().intern("Frog");
    reg.interner_mut().intern("Rat");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pump_and_untap_tribe,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_and_untap_tribe(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let syms: Vec<_> = ["Bird", "Frog", "Otter", "Rat"]
        .iter()
        .filter_map(|s| reg.interner().lookup(s))
        .collect();
    if syms.is_empty() {
        return Vec::new();
    }
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(syms);
    let ids = script::ids_matching(state, &filter, trig.controller);
    ids.into_iter()
        .map(|id| {
            Effect::Sequence(vec![
                Effect::Pump {
                    target: id,
                    power: 1,
                    toughness: 1,
                    duration: Duration::EndOfTurn,
                    keywords: vec![],
                },
                Effect::Untap { target: id },
            ])
        })
        .collect()
}
