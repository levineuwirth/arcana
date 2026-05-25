//! Dwarven Vigilantes — `{2}{R}` 2/2 red Creature — Dwarf.
//! "Whenever this creature attacks and isn't blocked, you may have it deal damage equal to its power
//! to target creature. If you do, this creature assigns no combat damage this turn."
//! GAP: trigger — no 'attacks and isn't blocked' condition; using SelfAttacks as closest.
//! GAP: effect — 'deal damage equal to power' is dynamic; using script::power_of on source.
//! GAP: 'assigns no combat damage this turn' prevention not expressible.

use arcana_core::effects::{Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dwarven Vigilantes");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — no 'attacks and isn't blocked' condition; using SelfAttacks as closest
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_unblocked_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn attacks_unblocked_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let arcana_core::targets::TargetChoice::Object(id) = target else { return Vec::new(); };
    let power = script::power_of(state, trig.source).max(0) as u32;
    // GAP: 'assigns no combat damage this turn' not expressible
    vec![Effect::DealDamage { target: DamageTarget::Object(*id), amount: power, source: trig.source }]
}
