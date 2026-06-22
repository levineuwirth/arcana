//! Steady-Handed Mook — `{2}{B}` 1/1 Creature — Human Rigger.
//! "Deathtouch"
//! "When this creature enters, it assembles a Contraption."
//!
//! Decomposition:
//! - Keyword line: Deathtouch. (Scryfall also tags "Assemble", which is not in
//!   the usable keyword surface — omitted.)
//! - "When this creature enters, it assembles a Contraption." → a SelfEnters
//!   triggered ability.
//!   GAP: the Contraption / assemble mechanic (Un-set) has no expressible
//!   Effect, so the effect body returns an empty vec.

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
    let name = reg.interner_mut().intern("Steady-Handed Mook");
    let human = reg.interner_mut().intern("Human");
    let rigger = reg.interner_mut().intern("Rigger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rigger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: keyword `Assemble` not in usable KeywordAbility surface.
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: assemble_contraption,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// GAP: "it assembles a Contraption" — the Contraption / assemble mechanic has
/// no expressible Effect.
fn assemble_contraption(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
