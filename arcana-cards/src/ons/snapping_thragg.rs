//! Snapping Thragg — `{4}{R}` 3/3 Beast.
//! "Whenever this creature deals combat damage to a player, you may have it
//!  deal 3 damage to target creature that player controls."
//! "Morph {4}{R}{R}."
//!
//! The combat-damage trigger targets a creature and deals 3 damage to it. The
//! "you may" is a resolution-time choice (not an intervening-if). The printed
//! "creature THAT PLAYER controls" controller restriction is keyed on the
//! damaged player (dynamic) and cannot be baked into the static target filter;
//! the target is therefore declared as any creature (a fidelity gap noted
//! below). Morph is not in the available keyword surface — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Snapping Thragg");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keyword Morph is not in the available KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: arcana_core::targets::ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: deal_three_to_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // GAP: "creature THAT PLAYER controls" controller restriction is
            // keyed on the dynamic damaged player and cannot be expressed in a
            // static target filter — declared as target creature.
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: arcana_core::targets::TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn deal_three_to_creature(
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
        amount: 3,
    }]
}
