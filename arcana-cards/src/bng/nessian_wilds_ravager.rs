//! Nessian Wilds Ravager — `{4}{G}{G}` 6/6 Creature — Hydra (green).
//!
//! * Tribute 6 — Tribute is not a usable KeywordAbility variant and its
//!   ETB +1/+1-counters-or-not choice is not expressible. GAP'd (the
//!   intervening-if "if tribute wasn't paid" likewise has no predicate).
//! * "When this creature enters, if tribute wasn't paid, you may have this
//!   creature fight another target creature." — modeled as a SelfEnters
//!   trigger that fights a target creature. GAP: the "if tribute wasn't
//!   paid" gate and the "you may" optionality are not expressible, so the
//!   fight is applied when a legal target is chosen.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nessian Wilds Ravager");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::SelfEntersBattlefield,
        // GAP: intervening-if "if tribute wasn't paid" not expressible.
        intervening_if: None,
        effect: fight_target,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: vec![TargetRequirement::target_creature()],
    }))
}

fn fight_target(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Fight {
        a: trig.source,
        b: *id,
    }]
}
