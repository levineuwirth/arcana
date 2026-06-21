//! Clement, the Worrywort — `{1}{G}{U}` 3/3 Legendary Frog Druid with
//! Vigilance.
//!
//! * Vigilance (keyword).
//! * Whenever Clement or another creature you control enters, return up
//!   to one target creature you control with lesser mana value to its
//!   owner's hand.  ("lesser mana value" gating is GAP'd — no dynamic
//!   per-trigger MV comparison in a target filter.)
//! * Frogs you control have "{T}: Add {G} or {U}…". — a static
//!   ability-granting effect to other permanents (GAP).

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
    let name = reg.interner_mut().intern("Clement, the Worrywort");
    let frog = reg.interner_mut().intern("Frog");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: bounce_lesser_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn bounce_lesser_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "with lesser mana value" — comparing the chosen target's MV to the
    // entering creature's MV is not expressible in the target filter; any
    // creature you control is targetable here.
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnToHand { target: *id }]
}
