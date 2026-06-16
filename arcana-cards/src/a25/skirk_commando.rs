//! Skirk Commando — `{1}{R}{R}` 2/1 red Goblin.
//!
//! Rules text:
//! * "Whenever this creature deals combat damage to a player, you may have it
//!   deal 2 damage to target creature that player controls." — a combat-damage
//!   triggered ability with a creature target.
//! * "Morph {2}{R}" — Morph is not part of the usable `KeywordAbility` surface
//!   for this card class, so it is GAP'd (see doc note below).
//!
//! GAP: Morph {2}{R} — Morph is not an available `KeywordAbility` variant and the
//! face-down-cast / turn-face-up mechanic is not expressible with the demonstrated
//! API. The keyword line is therefore emitted empty.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skirk Commando");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Morph {2}{R} not expressible (no Morph KeywordAbility variant).
        keywords: vec![],
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
            effect: deal_two_to_target_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

/// Combat-damage trigger: "you may have it deal 2 damage to target creature that
/// player controls." The "may" and the chosen creature are resolution-time
/// choices; the engine offers the target (UpTo(1)) so the optionality is honored.
fn deal_two_to_target_creature(
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
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Object(*id),
        amount: 2,
    }]
}
