//! God-Eternal Kefnet — `{2}{U}{U}` 4/5 Legendary Zombie God with Flying.
//!
//! Oracle:
//! * Flying.
//! * "You may reveal the first card you draw each turn ... copy that
//!   card ... costs {2} less." — a complex draw-replacement /
//!   reveal-and-copy ability with no matching primitive; GAP'd.
//! * "When God-Eternal Kefnet dies or is put into exile from the
//!   battlefield, you may put it into its owner's library third from
//!   the top." — a self-relocation on death/exile with a precise
//!   library position; no Effect for "put N from the top"; GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("God-Eternal Kefnet");
    let zombie = reg.interner_mut().intern("Zombie");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "You may reveal the first card you draw each turn ... copy
    // that instant/sorcery card ... that copy costs {2} less" — a
    // reveal-on-draw / copy-and-cost-reduce ability with no expressible
    // trigger condition (per-turn first-draw reveal) or effect.
    reg.register(
        CardDefinition::new(name, chars)
            // "When God-Eternal Kefnet dies ..." — the dies side fires,
            // but the effect (put it third from top of owner's library)
            // is not expressible. Emit the trigger with an empty effect.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_relocate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_relocate(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may put it into its owner's library third from the top" —
    // no Effect places a card at a specific library depth.
    Vec::new()
}
