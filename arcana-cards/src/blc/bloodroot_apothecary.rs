//! Bloodroot Apothecary — `{2}{G}` 3/3 green Squirrel Druid.
//!
//! Oracle:
//! Toxic 2
//! When this creature enters, you and target opponent each create a Treasure
//! token.
//! Whenever an opponent sacrifices a noncreature token, that player gets two
//! poison counters.
//!
//! Decomposition:
//! * Toxic 2 → `KeywordAbility::Toxic(2)`.
//! * ETB → you and the target opponent each create a Treasure token (wired via
//!   two `CreateCommodityToken` effects; the trigger targets an opponent).
//! * "opponent sacrifices a noncreature token → that player gets two poison
//!   counters" → the `Sacrificed` trigger is wired, but putting poison
//!   counters on a PLAYER is not an expressible effect (`AddCounters` is
//!   object-only; poison is otherwise applied via Toxic/Infect combat damage),
//!   so the effect body is GAP'd.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodroot Apothecary");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Toxic(2)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_treasures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::new()
                        .without_types(TypeLine::CREATURE.into())
                        .tokens_only()
                        .controlled_by(ControllerConstraint::Opponent),
                },
                intervening_if: None,
                effect: opponent_token_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_treasures(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }];
    if let Some(TargetChoice::Player(p)) = trig.targets.targets.first() {
        effects.push(Effect::CreateCommodityToken {
            controller: *p,
            kind: CommodityToken::Treasure,
            count: 1,
        });
    }
    effects
}

fn opponent_token_sacrifice(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that player gets two poison counters" — putting poison counters on
    // a PLAYER is not an expressible effect (`AddCounters` is object-only).
    Vec::new()
}
