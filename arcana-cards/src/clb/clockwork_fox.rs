//! Clockwork Fox — `{3}` 3/2 colorless Artifact Creature — Fox.
//! "When this creature leaves the battlefield, you draw two cards and each opponent
//! draws a card."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clockwork Fox");
    let fox = reg.interner_mut().intern("Fox");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: no "leaves the battlefield" trigger condition; using ZoneChange from
                // Battlefield to any zone as the closest approximation. Using Graveyard as to.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: arcana_core::targets::ObjectFilter::new()
                        .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: on_leaves,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_leaves(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::DrawCards { player: trig.controller, count: 2 }];
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::DrawCards { player: opp, count: 1 });
    }
    effects
}
