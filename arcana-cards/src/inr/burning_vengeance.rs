//! Burning Vengeance — `{2}{R}` enchantment (Innistrad, 2011).
//! "Whenever you cast a spell from your graveyard, this enchantment
//! deals 2 damage to any target."
//!
//! Wired on `SpellCastFromZone { caster: You, from_zone: Graveyard(0) }`
//! (CR 601.2a) — fires on flashback / escape / jump-start casts from your
//! graveyard. The 2-damage any-target payoff is wired faithfully.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Burning Vengeance");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // "Whenever you cast a spell from your graveyard."
                trigger_condition: TriggerCondition::SpellCastFromZone {
                    filter: None,
                    caster: ControllerConstraint::You,
                    from_zone: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: vengeance_bolt,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            },
        ),
    )
}

/// "…this enchantment deals 2 damage to any target."
fn vengeance_bolt(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let target = match trig.targets.targets.first() {
        Some(TargetChoice::Object(id)) => DamageTarget::Object(*id),
        Some(TargetChoice::Player(p)) => DamageTarget::Player(*p),
        Some(TargetChoice::ObjectOrPlayer(op)) => match op {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        target,
        amount: 2,
        source: trig.source,
    }]
}
