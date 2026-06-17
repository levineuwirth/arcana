//! Gev, Scaled Scorch — `{B}{R}` 3/2 Legendary Lizard Mercenary.
//! "Ward—Pay 2 life.
//!  Other creatures you control enter with an additional +1/+1 counter on them
//!   for each opponent who lost life this turn.
//!  Whenever you cast a Lizard spell, Gev deals 1 damage to target opponent."
//!
//! Ward—Pay 2 life is a non-mana ward, not expressible as KeywordAbility::Ward
//! (which takes a ManaCost). The enters-with-extra-counters static has no hook.
//! The Lizard-spell cast trigger is wired.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gev, Scaled Scorch");
    let lizard = reg.interner_mut().intern("Lizard");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(mercenary);

    let lizard_spell = script::subtype_filter(reg, "Lizard");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: keyword "Ward—Pay 2 life" — non-mana ward; KeywordAbility::Ward takes a ManaCost only.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "Other creatures you control enter with an additional +1/+1 counter
    // on them for each opponent who lost life this turn" — replacement-effect static, no hook.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(lizard_spell),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: damage_target_opponent,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::Exactly(1),
                controller: Some(ControllerConstraint::Opponent),
            }],
        }),
    )
}

/// Gev deals 1 damage to target opponent.
fn damage_target_opponent(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(*p),
        amount: 1,
    }]
}
