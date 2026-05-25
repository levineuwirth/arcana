//! Satyr Firedancer — `{1}{R}` 1/1 red Enchantment Creature — Satyr.
//! "Whenever an instant or sorcery spell you control deals damage to
//! an opponent, this creature deals that much damage to target creature
//! that player controls."
//! GAP: trigger condition — "when an instant/sorcery you control deals
//! damage to an opponent" has no exact TriggerCondition match;
//! using DamageDealt with opponent target as closest approximation.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Satyr Firedancer");
    let satyr = reg.interner_mut().intern("Satyr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(satyr);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "instant/sorcery you control deals damage to opponent"
                // — no exact variant; using DamageDealt as closest
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new().with_types(
                        TypeLine(TypeLine::INSTANT | TypeLine::SORCERY),
                    ),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: spell_damage_reflect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn spell_damage_reflect(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        target: DamageTarget::Object(*id),
        amount: n,
        source: trig.source,
    }]
}
