//! Tattermunge Duo — `{2}{R/G}` 2/3 Creature — Goblin Warrior Shaman.
//!
//! Oracle:
//! * Whenever you cast a red spell, this creature gets +1/+1 until end of turn.
//! * Whenever you cast a green spell, this creature gains forestwalk until end
//!   of turn.
//!
//! Decomposition: two color-filtered SpellCast triggers → two
//! `TriggeredAbilityDef`s, each acting on the source creature.

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
    let name = reg.interner_mut().intern("Tattermunge Duo");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);
    subtypes.0.insert(shaman);
    // Intern "Forest" so the forestwalk-grant resolver can recover the symbol.
    let _forest = reg.interner_mut().intern("Forest");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_colors(ColorSet::red())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pump_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_colors(ColorSet::green())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: grant_forestwalk,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn grant_forestwalk(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(forest) = reg.interner().lookup("Forest") else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::Landwalk(forest),
        duration: Duration::EndOfTurn,
    }]
}
