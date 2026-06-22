//! Quest Compleated Beast — `{2}{B}{B}` 4/4 Legendary Creature — Phyrexian
//! Beast. B. Menace, Lifelink, Toxic 2.
//! "Can't be blocked by tokens." — GAP: no self-static "can't be blocked by
//! [filter]".
//! "Your opponent can't gain life or remove poison counters." — GAP: no
//! static life-gain / counter-removal restriction effect.
//! "Whenever Quest Compleated Beast deals combat damage to an opponent, it
//! deals that much damage to target battle that player is protecting." —
//! DamageDealt (you, to a player, combat) → DealDamage of damage_amount() to
//! a target Battle. GAP (fidelity): the "that player is protecting"
//! protector linkage is not expressible — any Battle may be chosen.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quest Compleated Beast");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Menace,
            KeywordAbility::Lifelink,
            KeywordAbility::Toxic(2),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new().controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: damage_target_battle,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new().with_types(TypeLine::BATTLE.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn damage_target_battle(
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
    let n = trig.damage_amount().unwrap_or(0);
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Object(*id),
        amount: n,
    }]
}
