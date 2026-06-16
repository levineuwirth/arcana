//! Calamity, Galloping Inferno — `{4}{R}{R}` 4/6 legendary Horse Mount with
//! Haste.
//! "Whenever Calamity attacks while saddled, choose a nonlegendary creature
//!  that saddled it this turn and create a tapped and attacking token that's
//!  a copy of it. Sacrifice that token at the beginning of the next end step.
//!  Repeat this process once."
//! Saddle 1.
//!
//! GAP: Saddle is not a supported KeywordAbility variant — omitted; the
//! saddled-state tracking is unmodeled.
//! GAP (trigger effect): "create a tapped and attacking token that's a COPY
//! of a creature that saddled it" combines token-copy + tapped-and-attacking
//! + saddler tracking — no single primitive expresses a tapped/attacking
//! token COPY of a chosen permanent — body GAP'd. The attack trigger itself
//! is wired (best-effort) but cannot read "while saddled".

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
    let name = reg.interner_mut().intern("Calamity, Galloping Inferno");
    let horse = reg.interner_mut().intern("Horse");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: saddled_copy,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn saddled_copy(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: tapped-and-attacking token COPY of a saddler creature, repeated
    // once, with saddled-state gating — not expressible. See file doc.
    Vec::new()
}
