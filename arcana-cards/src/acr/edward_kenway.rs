//! Edward Kenway — `{2}{U}{B}{R}` 5/5 Legendary Human Assassin Pirate.
//!
//! Oracle:
//! * "At the beginning of your end step, create a Treasure token for each
//!   tapped Assassin, Pirate, and/or Vehicle you control." — a Treasure
//!   per matching tapped permanent (subtype OR over Assassin / Pirate /
//!   Vehicle), computed dynamically at resolution.
//! * "Whenever a Vehicle you control deals combat damage to a player, look
//!   at the top card of that player's library, then exile it face down.
//!   You may play that card for as long as it remains exiled." — the
//!   impulse-exile primitive (`ImpulseExile`) only operates on your OWN
//!   library, and there is no primitive to exile-with-play-permission a
//!   card from the damaged player's library. The effect body is GAP'd.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Edward Kenway");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);
    subtypes.0.insert(pirate);

    // Vehicle you control (combat-damage source).
    let vehicle_source = script::subtype_filter(reg, "Vehicle")
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_treasures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: vehicle_source,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: vehicle_steal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_treasures(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Tapped Assassin/Pirate/Vehicle you control — subtype OR rebuilt from
    // the read-only interner handle.
    let mut subs = Vec::new();
    for nm in ["Assassin", "Pirate", "Vehicle"] {
        if let Some(s) = reg.interner().lookup(nm) {
            subs.push(s);
        }
    }
    let filter = ObjectFilter::permanent()
        .controlled_by(ControllerConstraint::You)
        .tapped_only()
        .with_subtypes_any(subs);
    let n = script::count_matching(state, &filter, trig.controller);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: n,
    }]
}

fn vehicle_steal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at the top card of that player's library, then exile it
    // face down; you may play it for as long as it remains exiled" — no
    // primitive exiles-with-play-permission from the damaged player's
    // library (ImpulseExile is your-own-library only).
    Vec::new()
}
