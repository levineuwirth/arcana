//! Aquatic Alchemist // Bubble Up — `{1}{U}` Creature — Elemental 1/3 with
//! Adventure face "Bubble Up" (`{2}{U}` sorcery).
//!
//! Creature: Whenever you cast your first instant or sorcery spell each turn,
//! this creature gets +2/+0 until end of turn.
//! (GAP: "first instant or sorcery spell each turn" condition — no per-turn
//! spell-cast-count gate in triggered ability; wired as fires each time any
//! instant or sorcery is cast by you.)
//!
//! Adventure "Bubble Up": Put target instant or sorcery card from your
//! graveyard on top of your library.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aquatic Alchemist");
    let elemental_sub = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid creature cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Bubble Up");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid adventure cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Put target instant or sorcery card from your graveyard on top of your library."
            .into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Card {
                zone: Zone::Graveyard(0),
                filter: ObjectFilter::new().with_types_any(TypeLine(
                    TypeLine::INSTANT | TypeLine::SORCERY,
                )),
            },
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: bubble_up_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure)
            // Triggered ability: whenever you cast an instant or sorcery spell, +2/+0 until EOT.
            // GAP: "first spell each turn" gate not expressible; fires on each instant/sorcery cast.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().with_types_any(TypeLine(
                            TypeLine::INSTANT | TypeLine::SORCERY,
                        )),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pump_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn bubble_up_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::PutOnTopOfLibrary { target: *id }]
}

fn pump_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
