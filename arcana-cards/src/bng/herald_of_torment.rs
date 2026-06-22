//! Herald of Torment — `{1}{B}{B}` 3/3 Enchantment Creature — Demon with Flying.
//! Bestow {3}{B}{B}.
//! Flying.
//! At the beginning of your upkeep, you lose 1 life.
//! Enchanted creature gets +3/+3 and has flying.
//!
//! Flying is a base characteristic. The upkeep self-damage is a clean
//! `StepBegins { Upkeep, You }` → `Effect::LoseLife`. Bestow is not part of the
//! usable keyword surface (no exposed KeywordAbility::Bestow), and the
//! "enchanted creature gets +3/+3 and has flying" static only applies in the
//! bestowed-Aura mode — both GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Herald of Torment");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    // GAP: Bestow {3}{B}{B} — not part of the usable keyword surface; the
    // Aura-spell alternative cast and "becomes a creature again if unattached"
    // mechanic is unmodeled.
    // GAP: static "Enchanted creature gets +3/+3 and has flying" — only applies
    // when cast for its bestow cost as an Aura; the bestow attach is unmodeled.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: upkeep_lose_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn upkeep_lose_life(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::LoseLife {
        player: trig.controller,
        amount: 1,
    }]
}
