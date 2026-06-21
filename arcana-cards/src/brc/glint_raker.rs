//! Glint Raker — `{3}{U}` 1/3 Drake with Flying.
//!
//! Oracle:
//! * Flying
//! * This creature gets +X/+0, where X is the greatest mana value among
//!   artifacts you control. (static self-pump — GAP)
//! * Whenever this creature deals combat damage to a player, you may reveal
//!   that many cards from the top of your library. Put an artifact card
//!   revealed this way into your hand and the rest into your graveyard.
//!
//! Flying is a base keyword. The "+X/+0 where X = greatest mana value among
//! artifacts" static has no expressible greatest-mana-value primitive and is
//! GAP'd. The combat-damage trigger is modeled with DigTopN: look at the top
//! N cards (N = the combat damage dealt), optionally take one artifact card to
//! hand, and put the rest into the graveyard.

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glint Raker");
    let drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static — "This creature gets +X/+0, where X is the greatest mana
    // value among artifacts you control." No greatest-mana-value primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: dig_for_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dig_for_artifact(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DigTopN {
        player: trig.controller,
        count: n,
        filter: Some(ObjectFilter::new().with_types(TypeLine::ARTIFACT.into())),
        rest: DigRest::Graveyard,
    }]
}
