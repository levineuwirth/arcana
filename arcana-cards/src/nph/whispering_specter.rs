//! Whispering Specter — `{1}{B}{B}` 1/1 Phyrexian Specter with Flying and Infect.
//! "Flying. Infect.
//!  Whenever this creature deals combat damage to a player, you may sacrifice
//!  it. If you do, that player discards a card for each poison counter they
//!  have."
//!
//! Flying + Infect are base keywords. The combat-damage trigger condition is
//! recorded faithfully (`DamageDealt` to a player, combat only). Its payload
//! is GAP'd:
//! * GAP: "you may sacrifice it. If you do, that player discards a card for
//!   each poison counter they have." — there is no may-pay-a-cost-then gate
//!   for sacrificing the source on a trigger, and no `script::` accessor for
//!   a PLAYER's poison-counter count (poison is a player counter, not on a
//!   GameObject), so the dynamic discard count cannot be computed. Emitting a
//!   fixed discard would be a materially wrong card, so the whole payload is
//!   omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Whispering Specter");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let specter = reg.interner_mut().intern("Specter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(specter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Infect],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: combat_damage_payoff,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn combat_damage_payoff(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: may-sacrifice-then gate + per-poison-counter discard count (no
    // player poison-counter accessor in `script::`) — not expressible.
    Vec::new()
}
