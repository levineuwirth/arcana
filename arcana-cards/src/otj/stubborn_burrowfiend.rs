//! Stubborn Burrowfiend — `{1}{G}` 2/2 Badger Beast Mount.
//!
//! Oracle:
//! * "Whenever this creature becomes saddled for the first time each turn,
//!   mill two cards, then this creature gets +X/+X until end of turn, where
//!   X is the number of creature cards in your graveyard."
//! * Saddle 2 (Tap any number of other creatures you control with total
//!   power 2 or more: This Mount becomes saddled until end of turn.)
//!
//! GAP: Saddle and Mill keywords are not in the usable KeywordAbility surface,
//! and there is no "becomes saddled" TriggerCondition, so the saddle keyword
//! ability and its "becomes saddled" trigger are both unexpressible — emitted
//! as a stubbed trigger whose effect performs only the parts that are
//! expressible if it ever fires (mill + graveyard-scaled pump).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stubborn Burrowfiend");
    let badger = reg.interner_mut().intern("Badger");
    let beast = reg.interner_mut().intern("Beast");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(badger);
    subtypes.0.insert(beast);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: Saddle 2 / Mill are not in the usable KeywordAbility surface.
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: trigger condition "becomes saddled for the first time each
            // turn" has no matching TriggerCondition variant — there is no
            // Saddle event in the engine; the trigger cannot fire faithfully.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: saddled_mill_and_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn saddled_mill_and_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // X = number of creature cards in your graveyard.
    let x = script::graveyard_matching(
        state,
        &ObjectFilter::creature(),
        trig.controller,
        trig.controller,
    ) as i32;
    vec![
        Effect::Mill {
            player: trig.controller,
            count: 2,
        },
        Effect::Pump {
            target: trig.source,
            power: x,
            toughness: x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
    ]
}
