//! Watcher of the Wayside — `{3}` 3/2 colorless Artifact Creature — Golem.
//! "When this creature enters, target player mills two cards. You gain 2
//! life." ETB trigger with a single player target; resolver mills the
//! chosen player two and gains the controller 2 life.
//!
//! GAP: Scryfall lists `Mill` as a keyword, but it is not in the
//! engine's keyword enum — recorded via the triggered ability body
//! instead and `keywords` left empty.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Watcher of the Wayside");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill_and_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

/// ETB resolution: target player mills two cards; controller gains 2 life.
fn etb_mill_and_gain(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return vec![Effect::GainLife { player: trig.controller, amount: 2 }];
    };
    let TargetChoice::Player(p) = target else {
        return vec![Effect::GainLife { player: trig.controller, amount: 2 }];
    };
    vec![
        Effect::Mill { player: *p, count: 2 },
        Effect::GainLife { player: trig.controller, amount: 2 },
    ]
}
