//! Blim, Comedic Genius — `{2}{B}{R}` 4/3 Legendary Imp with Flying.
//! "Whenever Blim deals combat damage to a player, that player gains control
//!  of target permanent you control. Then each player loses life and discards
//!  cards equal to the number of permanents they control but don't own."
//!
//! The combat-damage trigger hands a target permanent you control to the
//! damaged player (Effect::ChangeControl). The "then each player loses life
//! and discards equal to permanents they control but don't own" rider needs a
//! "control-but-don't-own" count not available among the script helpers, so
//! it is GAP'd.

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
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blim, Comedic Genius");
    let imp = reg.interner_mut().intern("Imp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(imp);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: give_away_permanent,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn give_away_permanent(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ChangeControl {
        target: *id,
        new_controller: p,
    }]
    // GAP: "Then each player loses life and discards cards equal to the number
    // of permanents they control but don't own" — a "control-but-don't-own"
    // count is not available among the script helpers.
}
