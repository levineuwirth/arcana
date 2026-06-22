//! Enduring Friendship — `{1}{U}{R}` 2/2 red/blue Enchantment Creature —
//! Otter Glimmer.
//!
//! * Double team — GAP: not in the usable `KeywordAbility` surface.
//! * "Whenever you cast an instant or sorcery spell, creatures you control
//!   that are Otters and/or enchantments get +1/+1 until end of turn." —
//!   a SpellCast trigger pumping each matching creature you control.
//! * "When Enduring Friendship dies, if it was a creature, return it to the
//!   battlefield ... as an enchantment." — GAP: returning a creature as a
//!   non-creature enchantment face is not expressible with the available
//!   effects (`ReturnFromGraveyardToBattlefield` returns it unchanged; there
//!   is no "enters as an enchantment, not a creature" effect).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enduring Friendship");
    let otter = reg.interner_mut().intern("Otter");
    let glimmer = reg.interner_mut().intern("Glimmer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(otter);
    subtypes.0.insert(glimmer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Double team — not an available KeywordAbility variant.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pump_otters_and_enchantments,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_otters_and_enchantments(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Creatures you control that are Otters and/or enchantments.
    let otters = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Otter").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let enchantment_creatures = script::ids_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine(TypeLine::CREATURE | TypeLine::ENCHANTMENT))
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let mut ids = otters;
    for id in enchantment_creatures {
        if !ids.contains(&id) {
            ids.push(id);
        }
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}

fn dies_return(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "return it to the battlefield under its owner's control as an
    // enchantment (not a creature)". There is no effect to re-enter a
    // permanent stripped of its creature type; a plain return would bring
    // it back as a creature, which is wrong.
    Vec::new()
}
