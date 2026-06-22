//! Skyfire Kirin — `{2}{R}{R}` 3/3 Legendary Kirin Spirit with Flying.
//! "Whenever you cast a Spirit or Arcane spell, you may gain control of
//! target creature with that spell's mana value until end of turn."
//!
//! The trigger fires on casting a Spirit-or-Arcane spell (subtype-OR
//! filter). The control-steal is wired as Effect::ChangeControlEot.
//!
//! GAP (fidelity): the target restriction "with that spell's mana value"
//! is a dynamic, spell-mv-dependent constraint that a static target
//! filter cannot encode; the ability instead targets any creature.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyfire Kirin");
    let kirin = reg.interner_mut().intern("Kirin");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let arcane = reg.interner_mut().intern("Arcane");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kirin);
    subtypes.0.insert(spirit_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    let spell_filter = ObjectFilter::new().with_subtypes_any(vec![spirit_sub, arcane]);
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(spell_filter),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: steal_creature_eot,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn steal_creature_eot(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ChangeControlEot {
        target: *id,
        new_controller: trig.controller,
    }]
}
