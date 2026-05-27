//! Brambleguard Veteran — `{1}{G}{G}` 3/4 green Raccoon Warrior.
//! "Whenever you expend 4, Raccoons you control get +1/+1 and gain vigilance
//! until end of turn."
//! GAP: "expend 4" trigger (spending fourth total mana this turn) not
//! modeled as a TriggerCondition.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Brambleguard Veteran");
    let raccoon = reg.interner_mut().intern("Raccoon");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(raccoon);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "expend 4" trigger not available; using SpellCast as
                // nearest proxy to a mana-spending event.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: arcana_core::targets::ControllerConstraint::You,
                },
                intervening_if: None,
                effect: expend_pump_raccoons,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn expend_pump_raccoons(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "expend 4" not trackable; firing on every spell cast is approximate.
    let filter = script::subtype_filter(reg, "Raccoon");
    let ids = script::ids_matching(state, &filter, trig.controller);
    let mut effects: Vec<Effect> = ids.iter().map(|&id| Effect::Pump {
        target: id,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }).collect();
    for &id in &ids {
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}
