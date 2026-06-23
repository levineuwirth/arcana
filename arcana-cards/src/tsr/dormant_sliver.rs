//! Dormant Sliver — `{2}{G}{U}` 2/2 Creature — Sliver.
//!
//! Oracle:
//! * "All Sliver creatures have defender." — a static keyword-granting lord
//!   effect, wired via an ETB-installed `ContinuousEffect::filtered_keyword`
//!   grant of `KeywordAbility::Defender` to all Sliver creatures (unscoped —
//!   all players'), lasting `Duration::WhileSourceOnBattlefield`. "Sliver" is
//!   interned at register time so the effect fn can look it up immutably.
//! * "All Slivers have 'When this permanent enters, draw a card.'" — a static
//!   that grants a *triggered ability* to a whole tribe; only keyword grants
//!   are expressible (no grant-an-ability-to-other-permanents primitive in the
//!   allowed surface), so this half remains GAP'd.

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
    let name = reg.interner_mut().intern("Dormant Sliver");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static — "All Slivers have 'When this permanent enters, draw a
    // card.'" Granting a triggered ability to a whole tribe is not expressible
    // with the allowed continuous-effect surface (only keyword grants exist).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_sliver_defender,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "all Sliver creatures have defender", lasting until
/// this Sliver leaves the battlefield.
fn etb_install_sliver_defender(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let sliver = reg
        .interner()
        .lookup("Sliver")
        .expect("Sliver interned at register");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            ObjectFilter::creature().with_subtype_sym(sliver),
            KeywordAbility::Defender,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
