//! A-Binding Geist // A-Spectral Binding — {1}{U} Creature — Spirit (front) 2/1 (blue)
//! Whenever Binding Geist attacks, target creature an opponent controls gets -2/-0 until your next turn.
//! Disturb {U} (cast from graveyard transformed)
//! ---
//! Back: Enchantment — Aura
//! Enchant creature. Enchanted creature gets -2/-0.
//! If Spectral Binding would be put into a graveyard from anywhere, exile it instead.
//! GAP: Disturb (cast from graveyard transformed for {U}) not modeled.
//! GAP: Back face "enchant creature" aura attachment not modeled (back-face-only).
//! GAP: Back face "enchanted creature gets -2/-0" continuous effect not modeled.
//! GAP: Back face "if put into a graveyard, exile instead" replacement effect not modeled.
//! GAP: "-2/-0 until your next turn" duration — EndOfTurn is used as approximation
//!      (true "until your next turn" duration not available in Duration enum).
//! GAP: back-face-only triggered abilities not auto-installed on transform.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Binding Geist");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("A-Spectral Binding");
    let aura_sub = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura_sub);

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
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: binding_geist_attacks,
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
            }),
    )
}

fn binding_geist_attacks(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: duration should be "until your next turn"; using EndOfTurn as approximation.
    vec![Effect::Pump {
        target: *id,
        power: -2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
