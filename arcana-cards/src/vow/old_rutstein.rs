//! Old Rutstein — `{1}{B}{G}` 1/4 legendary black-green Human Peasant.
//! "When Old Rutstein enters and at the beginning of your upkeep, mill
//! a card. If a land card is milled this way, create a Treasure token.
//! If a creature card is milled this way, create a 1/1 green Insect
//! creature token. If a noncreature, nonland card is milled this way,
//! create a Blood token."
//! GAP: conditional branching on what type of card was milled is not
//! expressible via the Effect catalog; only the mill is emitted.
//! Two triggers are registered (ETB and upkeep).

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
    let name = reg.interner_mut().intern("Old Rutstein");
    let human = reg.interner_mut().intern("Human");
    let peasant = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(peasant);
    let _treasure = reg.interner_mut().intern("Treasure");
    let _insect = reg.interner_mut().intern("Insect");
    let _blood = reg.interner_mut().intern("Blood");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: mill_and_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: mill_and_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn mill_and_token(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: conditional token creation based on what card type was milled
    // — no engine Effect for "if the milled card was a [type]" branching
    vec![Effect::Mill { player: trig.controller, count: 1 }]
}
