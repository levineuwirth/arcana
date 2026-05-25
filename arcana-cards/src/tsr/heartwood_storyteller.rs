//! Heartwood Storyteller — `{1}{G}{G}` 2/3 green Creature — Treefolk.
//! "Whenever a player casts a noncreature spell, each of that player's opponents may draw a card."
//!
//! # GAP: "each of that player's opponents may draw a card" — identifying the caster's opponents
//! requires knowing who cast the spell; using trig.controller's opponents as approximation.

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
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heartwood Storyteller");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        types_any: Some(TypeLine(
                            TypeLine::INSTANT | TypeLine::SORCERY | TypeLine::ENCHANTMENT | TypeLine::ARTIFACT | TypeLine::LAND
                        )),
                        ..Default::default()
                    }),
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: noncreature_cast_opponents_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn noncreature_cast_opponents_draw(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each of that player's opponents" — using controller's opponents as approximation.
    let opponents = script::opponents(state, trig.controller);
    let effects: Vec<Effect> = opponents.into_iter().map(|p| {
        Effect::DrawCards { player: p, count: 1 }
    }).collect();
    vec![Effect::Sequence(effects)]
}
