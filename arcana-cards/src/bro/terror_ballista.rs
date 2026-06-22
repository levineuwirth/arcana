//! Terror Ballista — `{7}` 5/3 Artifact Creature — Construct with Menace.
//! Whenever this creature attacks, you may sacrifice another creature. When
//! you do, destroy target creature an opponent controls.
//! Unearth {3}{B}{B}.
//!
//! The attack trigger's "you may sacrifice another creature → reflexive
//! destroy" has no expressible shape (OptionalPayment only takes mana/life
//! costs, not a sacrifice cost), so it is GAP'd. Unearth is modeled as a
//! graveyard activated ability ({3}{B}{B}: return this card from your
//! graveyard to the battlefield); the "gains haste / exile at next end step
//! or if it would leave" riders are GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Terror Ballista");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_sac_destroy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Unearth {3}{B}{B} ({3}{B}{B}: Return this card from your graveyard to the battlefield.)".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: unearth_return,
            }),
    )
}

fn attack_sac_destroy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice another creature. When you do, destroy target
    // creature an opponent controls" — no sacrifice-as-cost reflexive shape.
    Vec::new()
}

fn unearth_return(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "gains haste; exile at the next end step or if it would leave" riders.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
