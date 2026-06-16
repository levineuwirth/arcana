//! Thief of Existence — `{1}{C}{G}` 3/4 Eldrazi with Devoid (colorless).
//! "When you cast this spell, exile up to one target noncreature, nonland
//!  permanent an opponent controls with mana value 4 or less. If you do,
//!  Thief of Existence gains 'When this creature leaves the battlefield,
//!  target opponent draws a card.'"
//!
//! Devoid has no KeywordAbility variant; it is modeled purely as the
//! colorless `ColorSet` (the card has no color). The cast trigger exiles
//! the chosen permanent; the "gains an ability" rider is GAP'd because the
//! grant is conditional on a successful exile and there is no engine hook
//! to bind a triggered ability only when the optional target was exiled.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
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
    let name = reg.interner_mut().intern("Thief of Existence");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{C}{G}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: on_cast_exile,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .without_types(TypeLine(TypeLine::CREATURE | TypeLine::LAND))
                        .controlled_by(ControllerConstraint::Opponent)
                        .with_max_cmc(4),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn on_cast_exile(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "If you do, ~ gains 'When this creature leaves the battlefield,
    // target opponent draws a card.'" — the grant is conditional on a
    // successful exile and there is no hook to bind the rider only on success.
    vec![Effect::ExilePermanent { target: *id }]
}
