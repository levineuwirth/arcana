//! Heartwood Storyteller — `{1}{G}{G}` 2/3 green Treefolk creature.
//! "Whenever a player casts a noncreature spell, each of that player's opponents may draw a card."

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
                    filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: noncreature_opponents_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn noncreature_opponents_draw(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // "each of that player's opponents may draw a card" — using controller's opponents.
    // GAP: should use the caster's opponents, not controller's; triggering_caster() not available
    // in this context for "Any" caster; approximating with controller's opponents.
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::DrawCards { player: p, count: 1 })
        .collect()
}
