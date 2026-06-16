//! Dauthi Mindripper — `{3}{B}` 2/1 black Dauthi Minion with Shadow.
//!
//! Oracle:
//! * Shadow (evergreen keyword).
//! * "Whenever this creature attacks and isn't blocked, you may
//!   sacrifice it. If you do, defending player discards three cards."
//!   Modeled with `SelfAttacksUnblocked`; the defending player discards
//!   three cards.
//!
//! GAP: the "you may sacrifice it. If you do, …" optional sacrifice
//! gate is not expressible (`OptionalPaymentKind` has only Mana/Life,
//! and there is no optional-sacrifice cost). The discard fires
//! unconditionally rather than being gated on paying the sacrifice.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Dauthi Mindripper");
    let dauthi = reg.interner_mut().intern("Dauthi");
    let minion = reg.interner_mut().intern("Minion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dauthi);
    subtypes.0.insert(minion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Shadow],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacksUnblocked,
            intervening_if: None,
            effect: defender_discards_three,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn defender_discards_three(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "you may sacrifice it. If you do" optional-sacrifice gate
    // isn't expressible; the discard fires unconditionally.
    let Some(p) = trig.defending_player() else { return Vec::new(); };
    vec![Effect::Discard {
        player: p,
        count: 3,
        choice: DiscardChoice::ControllerChooses,
    }]
}
