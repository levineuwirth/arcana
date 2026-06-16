//! Skanos Dragonheart — `{4}{G}` 4/4 Legendary Dragon Ranger.
//! "Whenever Skanos Dragonheart attacks, it gets +X/+X until end of turn, where
//!  X is the greatest power among other Dragons you control and Dragon cards in
//!  your graveyard."
//! "Choose a Background."
//!
//! The attack pump is modeled with X = the greatest power among Dragons you
//! control on the battlefield (computed via script). The "Dragon cards in your
//! graveyard" half is GAP'd (graveyard power-scan not expressible) as is the
//! exclusion of Skanos itself. "Choose a Background" is not in the usable
//! keyword surface and is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skanos Dragonheart");
    let dragon = reg.interner_mut().intern("Dragon");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(ranger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Choose a Background" is not in the usable keyword surface.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_pump(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // X = greatest power among Dragons you control on the battlefield.
    // GAP: "Dragon cards in your graveyard" half not counted.
    let filter = script::subtype_filter(reg, "Dragon").controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    let x = ids
        .into_iter()
        .map(|id| script::power_of(state, id).max(0))
        .max()
        .unwrap_or(0);
    if x == 0 {
        return Vec::new();
    }
    vec![Effect::Pump {
        target: trig.source,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
