//! Attentive Skywarden — `{2}{W}` 2/2 white Phyrexian Kor with Flying.
//!
//! Oracle:
//! * Flying (keyword). (Transform is a DFC marker, not a `KeywordAbility`
//!   variant, so it is not listed in `keywords`.)
//! * Whenever this creature deals combat damage to a player or battle,
//!   transform up to one target Incubator token you control.
//!
//! "or battle" is a fidelity gap — only the player side of the combat-damage
//! trigger is modeled (TargetFilter::Player).

use arcana_core::effects::{Effect, KeywordAbility};
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Attentive Skywarden");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let kor = reg.interner_mut().intern("Kor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(kor);

    let incubator_filter = script::subtype_filter(reg, "Incubator")
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    // Self-combat-damage trigger; the catalog convention is a
                    // broad source filter (no self-only DamageDealt scoping).
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: transform_incubator,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(incubator_filter),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn transform_incubator(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Transform { target: *id }]
}
