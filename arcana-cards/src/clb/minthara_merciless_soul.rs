//! Minthara, Merciless Soul — `{2}{W}{B}` 2/2 Legendary Elf Cleric.
//! Ward {X} (X = experience counters). At your end step, if a permanent
//! you controlled left the battlefield this turn, gain an experience
//! counter. Creatures you control get +1/+0 per experience counter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Minthara, Merciless Soul");
    let elf = reg.interner_mut().intern("Elf");
    let cleric = reg.interner_mut().intern("Cleric");
    let _experience = reg.interner_mut().intern("experience");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(cleric);

    // GAP: Ward {X} where X is a dynamic count (experience counters) — only a
    // fixed ManaCost Ward is expressible; a variable ward cost is not.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            // GAP: intervening-if "if a permanent you controlled left the
            // battlefield this turn" — no conditions helper for this; fired
            // unconditionally rather than baking a wrong check.
            intervening_if: None,
            effect: gain_experience,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
    // GAP: static "Creatures you control get +1/+0 for each experience counter
    // you have" — a dynamic continuous anthem, not a triggered/activated ability.
}

fn gain_experience(_state: &GameState, _trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let experience = reg.interner().lookup("experience").unwrap_or_default();
    // Experience counters live on the player; modeled as a named counter on the
    // source's controller via AddCounters on the source is not a player counter.
    // GAP: experience counters are a PLAYER counter; AddCounters targets objects,
    // so there is no expressible "you get an experience counter" effect.
    let _ = experience;
    Vec::new()
}
