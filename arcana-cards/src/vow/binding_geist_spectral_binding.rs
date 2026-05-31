//! Binding Geist // Spectral Binding — `{2}{U}` transforming DFC.
//! Front (Binding Geist — Creature — Spirit, 3/1):
//!   Whenever this creature attacks, target creature an opponent controls
//!   gets -2/-0 until end of turn.
//!   Disturb {1}{U}.
//! Back (Spectral Binding — Enchantment — Aura):
//!   Enchant creature. Enchanted creature gets -2/-0.
//!   If Spectral Binding would be put into a graveyard from anywhere, exile it instead.
//!
//! GAPs:
//! - Disturb {1}{U}: the alternate graveyard-cast-transformed mechanic is not
//!   in the usable keyword surface; not modeled.
//! - Back face Aura static "-2/-0" and the "exile instead of graveyard"
//!   replacement are static/aura abilities on the back face; not expressible
//!   here (no static-modifier or self-replacement primitive on a face). The
//!   back face is still declared so transform has a target.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Binding Geist");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Spectral Binding — Enchantment — Aura.
    let back_name = reg.interner_mut().intern("Spectral Binding");
    let aura = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front (face 0): on attack, target creature an opponent controls
            // gets -2/-0 until end of turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: weaken_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_trigger_face_gate(1, 0),
    )
}

fn weaken_target(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: -2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
