//! Congregation Gryff — `{1}{G}{W}` 1/4 Hippogriff Mount with Flying
//! and Lifelink.
//! "Whenever this creature attacks while saddled, it gets +X/+X until
//! end of turn, where X is the number of Mounts you control."
//! "Saddle 3."
//!
//! Flying and Lifelink are base keywords. Saddle is not a
//! `KeywordAbility` variant and there is no saddled-state tracking, so
//! the Saddle ability is GAP'd. The attack trigger's "while saddled"
//! gate cannot be tested (no saddled flag), so its conditional pump is
//! GAP'd rather than over-fired on every attack — the `SelfAttacks`
//! condition is still wired so the catalog records the shape.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Congregation Gryff");
    let hippogriff = reg.interner_mut().intern("Hippogriff");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hippogriff);
    subtypes.0.insert(mount);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };
    // GAP (Saddle 3): "Tap other creatures with total power 3+: this Mount
    // becomes saddled until end of turn." Saddle is not a KeywordAbility
    // variant and there is no saddled-state to set.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: gap_saddled_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gap_saddled_pump(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "while saddled" cannot be tested (no saddled-state flag). Firing
    // the +X/+X pump unconditionally would materially over-fire, so the
    // conditional payload is omitted.
    Vec::new()
}
