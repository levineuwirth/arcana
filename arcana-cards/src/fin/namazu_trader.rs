//! Namazu Trader — `{3}{B}` 3/4 Fish Citizen.
//! "When this creature enters, you lose 1 life and create a Treasure
//! token."
//! "Whenever this creature attacks, you may sacrifice another creature
//! or artifact. If you do, surveil 2."
//!
//! (Surveil and Treasure are reminder/mechanic words, not KeywordAbility
//! variants — no keywords vec.)
//!
//! ETB trigger is fully wired. The attack trigger is GAP'd: "you may
//! sacrifice another creature or artifact. If you do, surveil 2" is an
//! optional sacrifice gate, but OptionalPaymentKind only supports Mana /
//! Life (no Sacrifice cost), so the may-sacrifice-then-surveil link is
//! unexpressible.

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
    let name = reg.interner_mut().intern("Namazu Trader");
    let fish = reg.interner_mut().intern("Fish");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);
    subtypes.0.insert(citizen);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_lose_life_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_may_sac_surveil,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_lose_life_treasure(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::LoseLife { player: trig.controller, amount: 1 },
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Treasure,
            count: 1,
        },
    ]
}

fn attack_may_sac_surveil(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may sacrifice another creature or artifact. If you do,
    // surveil 2" — OptionalPaymentKind has no Sacrifice variant, so the
    // optional-sacrifice-then-surveil link is unexpressible.
    Vec::new()
}
