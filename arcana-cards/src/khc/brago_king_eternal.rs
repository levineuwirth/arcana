//! Brago, King Eternal — `{2}{W}{U}` 2/4 white-blue legendary Spirit Noble.
//!
//! Flying.
//! Whenever Brago deals combat damage to a player, exile any number of target
//!   nonland permanents you control, then return those cards to the battlefield
//!   under their owner's control.
//!   (GAP: the immediate "then return those cards" half of the blink is not in
//!   the authorized effect catalog — re-id on the zone move makes a same-id
//!   ReturnFromExileToBattlefield a no-op — so only the exile is emitted.
//!   Matches the catalog convention for "whenever ~ deals combat damage to a
//!   player" using a broad DamageDealt source_filter.)

use arcana_core::effects::{Effect, KeywordAbility};
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brago, King Eternal");
    let spirit = reg.interner_mut().intern("Spirit");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: blink_your_permanents,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .without_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Any,
                controller: None,
            }],
        }),
    )
}

fn blink_your_permanents(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "then return those cards to the battlefield" — immediate blink return
    // not expressible (re-id on zone move). Emit the exile of each chosen target.
    trig.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::ExilePermanent { target: *id }),
            _ => None,
        })
        .collect()
}
