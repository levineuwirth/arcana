//! Garna, the Bloodflame — `{3}{B}{R}` 3/3 Legendary Human Warrior with Flash.
//!
//! "When Garna enters, return to your hand all creature cards in your graveyard
//!  that were put there from anywhere this turn." — no per-turn graveyard-arrival
//!  filter / mass-return effect (GAP).
//! "Other creatures you control have haste." — WIRED as an ETB-installed
//!  `ContinuousEffect::keyword_anthem(Haste)`, lasting while Garna is on the
//!  battlefield (glorious_anthem precedent). The anthem grants haste to all your
//!  creatures; the "OTHER" self-inclusion is the documented minor fidelity gap.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garna, the Bloodflame");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: ETB mass-return of creatures put into the graveyard this turn — no
    // per-turn graveyard-arrival filter / mass-return effect.
    reg.register(
        CardDefinition::new(name, chars)
            // "Other creatures you control have haste." installed on ETB.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_haste_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Install "creatures you control have haste" anchored to Garna, lasting while
/// it remains on the battlefield.
fn install_haste_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::keyword_anthem(
            trig.source,
            trig.controller,
            KeywordAbility::Haste,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
