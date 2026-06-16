//! Cornered Crook — `{4}{R}` 5/4 red Lizard Warrior.
//! "When this creature enters, you may sacrifice an artifact. When you do, this creature
//! deals 3 damage to any target."
//! GAP: "you may sacrifice an artifact" as cost during trigger resolution — no sacrifice-cost
//! in triggered ability effect.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cornered Crook");
    let lizard = reg.interner_mut().intern("Lizard");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_deal_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }),
    )
}

fn etb_deal_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice an artifact" cost not modeled
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    match target {
        TargetChoice::Object(id) => vec![Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Object(*id),
            amount: 3,
        }],
        TargetChoice::Player(pid) => vec![Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(*pid),
            amount: 3,
        }],
        _ => Vec::new(),
    }
}
