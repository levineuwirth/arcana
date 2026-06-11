//! Food Coma — `{3}{W}` enchantment.
//! "When this enchantment enters, exile target creature an opponent
//! controls until this enchantment leaves the battlefield. Create a Food
//! token."
//!
//! ETB trigger targeting an opponent's creature: the exile and the Food
//! token are wired; the "until this enchantment leaves the battlefield"
//! return linkage is a documented GAP (the exile is unconditional).

use arcana_core::effects::{CommodityToken, Effect};
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
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Food Coma");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: exile_and_feed,
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

/// "…exile target creature an opponent controls until this enchantment
/// leaves the battlefield. Create a Food token."
fn exile_and_feed(
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
    // GAP: "until this enchantment leaves the battlefield" — the exiled
    // creature does not return when this enchantment leaves (no
    // linked-exile return primitive); the exile is permanent.
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Food,
            count: 1,
        },
    ]
}
