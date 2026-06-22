//! Gelatinous Cube — `{2}{B}{B}` 4/3 black Ooze.
//!
//! * Engulf — When this creature enters, exile target non-Ooze creature an
//!   opponent controls until this creature leaves the battlefield.
//! * Dissolve — `{X}{B}: Put target creature card with mana value X exiled with
//!   this creature into its owner's graveyard.`  (GAP'd.)
//!
//! Engulf is wired via the O-ring `Effect::ExileUntilSourceLeaves` linkage. The
//! "non-Ooze" subtype exclusion on the target is a fidelity GAP — the
//! demonstrated `ObjectFilter` refinement surface offers no subtype-exclusion
//! chainer, so the target is restricted only to "creature an opponent
//! controls". Dissolve is fully GAP'd: the `{X}` activation cost has no field,
//! and there is no target/effect for "a creature card exiled with this source"
//! moving to its owner's graveyard.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Gelatinous Cube");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Dissolve — {X}{B}: Put target creature card with mana value X
    // exiled with this creature into its owner's graveyard." The {X} activation
    // cost has no field, and no target/effect moves a card exiled-with-source
    // into a graveyard.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: engulf,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    // GAP: "non-Ooze" subtype exclusion not expressible via the
                    // demonstrated ObjectFilter refinements; target restricted
                    // to "creature an opponent controls".
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn engulf(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ExileUntilSourceLeaves {
        source: trig.source,
        target: *id,
    }]
}
