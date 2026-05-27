//! Ruination Rioter — `{R}{G}` 2/2 green/red Human Berserker.
//! "When this creature dies, you may have it deal damage to any target equal to the number of
//! land cards in your graveyard."
//! GAP: "number of land cards in your graveyard" — no script helper for counting land cards in
//! graveyard. Using graveyard_size as approximation.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ruination Rioter");
    let human = reg.interner_mut().intern("Human");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(berserker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: deal_damage_on_death,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }),
    )
}

fn deal_damage_on_death(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "number of land cards in your graveyard" — using total graveyard size as approximation
    let n = script::graveyard_size(state, trig.controller);
    if n == 0 {
        return Vec::new();
    }
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    match target {
        TargetChoice::Object(id) => vec![Effect::DealDamage {
            target: DamageTarget::Object(*id),
            amount: n,
            source: trig.source,
        }],
        TargetChoice::Player(p) => vec![Effect::DealDamage {
            target: DamageTarget::Player(*p),
            amount: n,
            source: trig.source,
        }],
        _ => Vec::new(),
    }
}
