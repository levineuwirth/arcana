//! Gilded Ghoda — `{1}{R}` 2/2 Creature — Horse Mount.
//!
//! * Whenever this creature attacks while saddled, create a Treasure token.
//! * Saddle 1 (tap other creatures with total power 1+: this Mount becomes
//!   saddled).
//!
//! Saddle/Mount have no usable `KeywordAbility` variant (not in the supported
//! keyword surface) and the "saddled" status isn't trackable in the engine, so
//! the Saddle activated ability is GAP'd. The attack trigger is wired as a
//! plain `SelfAttacks` minting a Treasure; the "while saddled" intervening
//! gate is a documented fidelity gap (it will fire on every attack).

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gilded Ghoda");
    let horse = reg.interner_mut().intern("Horse");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Saddle / Treasure keyword markers — no usable KeywordAbility
        // variants (Mount/Saddle unsupported).
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_make_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: "Saddle 1" activated ability — saddling (tap N other creatures
        // with total power ≥ 1) is not an expressible ActivationCost, and the
        // "saddled until end of turn" status is unmodeled.
    )
}

/// On attack: create a Treasure token. (The "while saddled" gate is a
/// fidelity gap — saddled status is not tracked, so this always fires.)
fn on_attack_make_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
