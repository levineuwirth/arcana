//! Laccolith Warrior — `{2}{R}{R}` 3/3 red Beast Warrior.
//! "Whenever this creature becomes blocked, you may have it deal damage equal to its power
//! to target creature. If you do, this creature assigns no combat damage this turn."
//!
//! # GAP: trigger — "whenever this creature becomes blocked" has no matching TriggerCondition
//! variant. Closest is SelfAttacks; using that with a GAP note.
//! GAP: "deal damage equal to its power" requires script::power_of at resolve time.
//! GAP: "this creature assigns no combat damage" is a replacement effect not modeled.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Laccolith Warrior");
    let beast = reg.interner_mut().intern("Beast");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever this creature becomes blocked" has no variant;
                // using SelfAttacks as closest approximation.
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: blocked_deal_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn blocked_deal_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let power = script::power_of(state, trig.source).max(0) as u32;
    // GAP: "this creature assigns no combat damage this turn" replacement effect not modeled.
    vec![Effect::DealDamage {
        target: DamageTarget::Object(*id),
        amount: power,
        source: trig.source,
    }]
}
