//! Troublemaker Ouphe — `{1}{G}` 2/2 Ouphe.
//!
//! * Bargain (sacrifice an artifact, enchantment, or token as you cast
//!   this spell). `Bargain` is not a usable `KeywordAbility` variant and
//!   the additional-cost machinery is not in the demonstrated surface,
//!   so it is not modeled — see GAP below.
//! * When this creature enters, IF it was bargained, exile target
//!   artifact or enchantment an opponent controls. The "if it was
//!   bargained" gate is a was-cast-with-additional-cost condition that
//!   is not expressible as an `intervening_if` predicate — GAP'd; the
//!   exile effect itself is emitted (it will fire unconditionally).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Troublemaker Ouphe");
    let ouphe = reg.interner_mut().intern("Ouphe");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ouphe);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: Bargain additional cost (sacrifice artifact/enchantment/token
    // as you cast) is not modeled; the ETB's "if it was bargained" gate
    // therefore cannot be checked — the exile fires unconditionally.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: exile_artifact_or_enchantment,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT))
                        .controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn exile_artifact_or_enchantment(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target: *id }]
}
