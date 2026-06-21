//! Oracle of the Alpha — `{2}{U}` 2/3 Bird Wizard with Flying.
//! "When Oracle of the Alpha enters the battlefield, conjure the Power
//! Nine into your library, then shuffle."
//! "Whenever Oracle of the Alpha attacks, scry 1."
//!
//! Flying is a base keyword. The ETB conjure is an Arena-only mechanic
//! with no `Effect::Conjure` variant — GAP'd. The attack trigger
//! scries 1.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oracle of the Alpha");
    let bird = reg.interner_mut().intern("Bird");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_conjure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_scry,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_conjure(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure not modeled (Arena-only mechanic; would need
    // registry-by-name lookup in Effect::execute). No Effect::Conjure variant.
    Vec::new()
}

fn attack_scry(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Scry { player: trig.controller, count: 1 }]
}
