//! Lavaleaper — `{3}{R}` 4/4 Elemental.
//! "All creatures have haste." — static keyword grant, wired as a
//! `SelfEntersBattlefield` trigger installing a board-wide
//! `ContinuousEffect::filtered_keyword` (filter = every creature) that
//! auto-expires when Lavaleaper leaves play.
//! "Whenever a player taps a basic land for mana, that player adds one
//!  mana of any type that land produced." (GAP — no "taps a land for
//!  mana" TriggerCondition, and no helper to read what a land produced.
//!  This is not the "any color" per-color-ability case: the produced
//!  type is dictated by the tapped land, not chosen by activating one of
//!  several abilities, and the trigger itself is unmodeled.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lavaleaper");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Whenever a player taps a basic land for mana, that player adds
    // one mana of any type that land produced" — no "tapped a land for mana"
    // TriggerCondition, and no helper to read what a land produced.
    reg.register(
        CardDefinition::new(name, chars)
            // "All creatures have haste."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_all_creatures_haste,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: install "all creatures have haste" board-wide, lasting until
/// Lavaleaper leaves the battlefield.
fn install_all_creatures_haste(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            ObjectFilter::creature(),
            KeywordAbility::Haste,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
