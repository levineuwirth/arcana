//! Mirkwood Bats — `{3}{B}` 2/3 Bat with Flying.
//! "Whenever you create or sacrifice a token, each opponent loses 1 life."
//! Only the SACRIFICE half is expressible (`Sacrificed { token filter }`);
//! GAP: there is no "create a token" trigger condition for the create half.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Mirkwood Bats");
    let bat = reg.interner_mut().intern("Bat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        // GAP: "create ... a token" half — no token-created trigger condition.
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::Sacrificed {
                filter: ObjectFilter::permanent()
                    .controlled_by(ControllerConstraint::You)
                    .tokens_only(),
            },
            intervening_if: None,
            effect: each_opponent_loses_1,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn each_opponent_loses_1(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let effects = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::LoseLife {
            player: p,
            amount: 1,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
