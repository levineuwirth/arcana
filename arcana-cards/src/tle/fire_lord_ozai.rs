//! Fire Lord Ozai — `{3}{B}` 4/4 Legendary Creature — Human Noble.
//!
//! * Whenever Fire Lord Ozai attacks, you may sacrifice another creature. If
//!   you do, add an amount of {R} equal to the sacrificed creature's power
//!   (mana doesn't empty as combat steps end).
//! * `{6}`: Exile the top card of each opponent's library. Until end of turn,
//!   you may play one of those cards without paying its mana cost.
//!
//! Both ability bodies are GAP'd. The attack trigger's payload mints mana
//! equal to the power of the *creature the player chooses to sacrifice* — a
//! sacrifice-cost-bound dynamic amount with combat-mana-retention rider that
//! cannot be expressed (the chosen creature's power is not a resolution-time
//! `script::` amount, and "don't lose this mana as steps end" has no primitive).
//! The activated ability exiles from *opponents'* libraries with free-cast
//! permission, which `ImpulseExile` (own-library only) cannot model.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fire Lord Ozai");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_sac_for_mana,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}: Exile the top card of each opponent's library. Until end of turn, you may play one of those cards without paying its mana cost.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_each_opponent_top,
            }),
    )
}

fn on_attack_sac_for_mana(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice another creature; if you do, add {R} equal to
    // its power; don't lose this mana as steps end." The mana amount depends
    // on the power of the player-chosen sacrificed creature (not a
    // resolution-time script amount), and the combat-step mana-retention
    // rider has no primitive.
    Vec::new()
}

fn exile_each_opponent_top(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile the top card of each OPPONENT's library with until-end-of-turn
    // free-cast permission. ImpulseExile only handles your own library; there
    // is no opponent-library exile-and-play primitive.
    Vec::new()
}
