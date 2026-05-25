//! Donal, Herald of Wings — `{2}{U}{U}` 3/3 blue Legendary Creature — Human Wizard.
//! "Whenever you cast a nonlegendary creature spell with flying, you may copy it, except
//! the copy is a 1/1 Spirit in addition to its other types. Do this only once each turn."
//!
//! # Notes
//! GAP: "copy the spell, except the copy is a 1/1 Spirit" — CopySpell effect exists but
//! modifying the copy's type/PT is not supported. Modeled as CopySpell without modification.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Donal, Herald of Wings");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        types_any: Some(TypeLine::CREATURE.into()),
                        ..Default::default()
                    }),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: flying_creature_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn flying_creature_copy(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "copy it as a 1/1 Spirit" — CopySpell doesn't support type/PT override.
    // GAP: filter for "nonlegendary + flying" creature spells — ObjectFilter can't check
    // supertypes or keywords on the stack object.
    vec![Effect::CopySpell { target: trig.source }]
}
