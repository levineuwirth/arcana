//! Teller of Tales — `{3}{U}{U}` 3/3 Spirit with Flying.
//! "Flying
//!  Whenever you cast a Spirit or Arcane spell, you may tap or untap
//!  target creature."
//!
//! Flying base keyword. The trigger watches your Spirit/Arcane spells
//! (subtype-OR filter) and targets a creature. "Tap OR untap" is a
//! player choice between two effects with no modal-trigger machinery, so
//! the resolver taps the target (the untap option / "may" is a
//! documented partial).

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teller of Tales");
    let spirit = reg.interner_mut().intern("Spirit");
    let _arcane = reg.interner_mut().intern("Arcane");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let spirit_sym = reg.interner_mut().intern("Spirit");
    let arcane_sym = reg.interner_mut().intern("Arcane");

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(
                    ObjectFilter::default()
                        .with_subtypes_any(vec![spirit_sym, arcane_sym]),
                ),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: tap_target_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn tap_target_creature(
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
    // Partial: "tap OR untap" — resolves as Tap (the untap branch and
    // the "may" are not modeled).
    vec![Effect::Tap { target: *id }]
}
