//! Fang, Roku's Companion — `{3}{R}{R}` 4/4 Legendary Dragon with Flying.
//! "Whenever Fang attacks, another target legendary creature you control
//!  gets +X/+0 until end of turn, where X is Fang's power."
//! "When Fang dies, if he wasn't a Spirit, return this card to the
//!  battlefield under your control. He's a Spirit in addition to his
//!  other types."
//!
//! Wired: Flying keyword; the attack pump (+X/+0 where X = Fang's power
//! computed at resolution) onto a target legendary creature you control.
//! The dies trigger is GAP'd — its "if he wasn't a Spirit" intervening
//! gate (a self-subtype predicate) has no condition helper, and a
//! return-self-with-permanent-type-add can't be expressed faithfully
//! without it.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
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
    let name = reg.interner_mut().intern("Fang, Roku's Companion");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_legendary,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You)
                            .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // GAP: "When Fang dies, if he wasn't a Spirit, return this
            // card to the battlefield ...; he's a Spirit in addition to
            // his other types." No self-subtype intervening-if helper;
            // return-self-with-permanent-AddType not faithfully
            // expressible.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_return_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_legendary(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let x = script::power_of(state, trig.source).max(0);
    vec![Effect::Pump {
        target: *id,
        power: x,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn dies_return_gap(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: conditional return-self-from-graveyard with permanent Spirit
    // type addition (no self-subtype intervening-if; no faithful path).
    Vec::new()
}
