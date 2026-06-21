//! Praetorhoof Behemoth — `{4}{G}{G}` 4/4 Phyrexian Praetor Beast with Haste.
//!
//! Oracle:
//! * Haste.
//! * "When Praetorhoof Behemoth enters, you may search your library
//!   and reveal any number of Praetors from among cards in your
//!   library and hand. Creatures you control gain trample and get
//!   +X/+X until end of turn, where X is the number of Praetors you
//!   have showing from among your hand, library, battlefield, and
//!   graveyard. Then shuffle." — a reveal-from-multiple-zones search
//!   feeding a cross-zone X count for a board-wide pump; neither the
//!   reveal nor the four-zone count is expressible, so the ETB trigger
//!   is emitted with an empty effect.

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
    let name = reg.interner_mut().intern("Praetorhoof Behemoth");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let praetor = reg.interner_mut().intern("Praetor");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(praetor);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_praetor_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_praetor_pump(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal-any-number-of-Praetors from library AND hand, then
    // pump all your creatures by +X/+X (and grant trample) where X is
    // the count of revealed Praetors across hand/library/battlefield/
    // graveyard. No primitive expresses the multi-zone reveal search
    // nor the four-zone dynamic X.
    Vec::new()
}
