//! Ashcoat of the Shadow Swarm — `{3}{B}` 3/4 Legendary Rat Warlock.
//! "Whenever Ashcoat attacks or blocks, other Rats you control get
//! +X/+X until end of turn, where X is the number of Rats you control."
//! "At the beginning of your end step, you may mill four cards. If you
//! do, return up to two Rat creature cards from your graveyard to your
//! hand."
//!
//! The "attacks or blocks" line is decomposed into two triggers
//! (SelfAttacks + SelfBlocks), each pumping every Rat you control by
//! +X/+X (X = number of Rats). Fidelity gap: the pump includes Ashcoat
//! itself ("other Rats" exclusion not expressible via ForEach).
//! The end-step mill/return ability is GAP'd (an optional "you may mill"
//! gated on a subsequent targeted graveyard return is not expressible).
//! The Mill keyword line is GAP'd (no `KeywordAbility::Mill`).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
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
    let name = reg.interner_mut().intern("Ashcoat of the Shadow Swarm");
    let rat = reg.interner_mut().intern("Rat");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Mill keyword line — no KeywordAbility::Mill variant.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_rats,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: pump_rats,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: end-step "you may mill four; if you do return up to two
        // Rat creature cards from your graveyard to your hand" — an
        // optional mill gating a subsequent targeted graveyard return
        // is not expressible. Whole ability omitted.
    )
}

fn pump_rats(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let rat_filter = script::subtype_filter(reg, "Rat").controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &rat_filter, trig.controller) as i32;
    let ids = script::ids_matching(state, &rat_filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: n,
            toughness: n,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
