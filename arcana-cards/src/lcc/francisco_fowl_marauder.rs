//! Francisco, Fowl Marauder — `{1}{B}` 0/1 Legendary Bird Pirate with Flying.
//!
//! Oracle:
//! Flying
//! Francisco can't block.
//! Whenever one or more Pirates you control deal damage to a player,
//!   Francisco explores.
//! Partner.
//!
//! Flying is emitted as a keyword. The damage trigger fires `Effect::Explore`
//! on Francisco himself.
//!
//! GAP: "Partner" and "Explore" (the Scryfall keyword line for the explore
//! ability) are not usable `KeywordAbility` variants — emitted as no keyword.
//! GAP: "Francisco can't block." is a pure static restriction with no
//! expressible primitive — dropped.
//! Note: the trigger fires per damage-dealing Pirate (the "one or more …
//! once" aggregation is a documented fidelity gap; explore-once-per-event is
//! the closest expressible behaviour).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Francisco, Fowl Marauder");
    let bird = reg.interner_mut().intern("Bird");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(pirate);

    let pirate_filter = arcana_core::script::subtype_filter(reg, "Pirate")
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: pirate_filter,
                target_filter: TargetFilter::Player,
                combat_only: false,
            },
            intervening_if: None,
            effect: pirates_deal_damage_explore,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn pirates_deal_damage_explore(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Explore {
        player: trig.controller,
        target: trig.source,
    }]
}
