//! Redoubled Stormsinger — `{2}{R}` 3/3 Orc Wizard with First strike.
//!
//! Oracle:
//! * First strike.
//! * Whenever this creature attacks, for each creature token you control
//!   that entered this turn, create a tapped and attacking token that's a
//!   copy of that token. At the beginning of the next end step, sacrifice
//!   those tokens.
//!
//! The keyword is a base characteristic. The attack trigger's payload
//! requires a "creature tokens you control that entered THIS TURN" set —
//! there is no exposed `script::` filter for "entered this turn", and the
//! per-token copy-tapped-attacking + delayed mass-sacrifice cannot be
//! assembled from the demonstrated primitives, so the trigger is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Redoubled Stormsinger");
    let orc = reg.interner_mut().intern("Orc");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_copy_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_copy_tokens(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "for each creature token you control that entered THIS TURN, create
    // a tapped-and-attacking copy of that token, then sacrifice those tokens
    // at the next end step" — no exposed filter for "entered this turn" and the
    // per-token copy + delayed mass-sacrifice cannot be assembled here.
    Vec::new()
}
