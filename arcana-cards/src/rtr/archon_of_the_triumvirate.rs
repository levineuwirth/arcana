//! Archon of the Triumvirate — `{5}{W}{U}` 4/5 Creature — Archon.
//! Flying.
//! "Whenever this creature attacks, detain up to two target nonland permanents
//! your opponents control." Detain (until your next turn: can't attack, can't
//! block, activated abilities can't be activated) has no single primitive; the
//! attack-can't / block-can't halves are approximated with ForbidAttacking +
//! ForbidBlocking until your next turn over each chosen target. The
//! "activated abilities can't be activated" half is GAP'd (no demonstrated
//! activation-lock effect).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Archon of the Triumvirate");
    let archon = reg.interner_mut().intern("Archon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(archon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: detain_targets,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .without_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::UpTo(2),
                controller: None,
            }],
        }),
    )
}

fn detain_targets(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Approximate detain: each chosen permanent can't attack or block until
    // our next turn. GAP: detain also stops activated abilities — not modeled.
    let mut effects = Vec::new();
    for target in trig.targets.targets.iter() {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::ForbidAttacking {
                target: *id,
                duration: Duration::UntilYourNextTurn(trig.controller),
            });
            effects.push(Effect::ForbidBlocking {
                target: *id,
                duration: Duration::UntilYourNextTurn(trig.controller),
            });
        }
    }
    effects
}
