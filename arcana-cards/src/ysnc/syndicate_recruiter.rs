//! Syndicate Recruiter — `{2}{U}{B}` 4/5 Vampire Rogue (B/U).
//!
//! Oracle:
//! * Flying, ward {1}  (keyword line — Ward {1} is `KeywordAbility::Ward`.)
//! * When Syndicate Recruiter enters the battlefield, mill four cards. Then if
//!   there are five or more mana values among cards in your graveyard, conjure
//!   a card named Dig Up the Body into your hand.  (Partial — the Mill 4 half
//!   is wired; the conditional "conjure a card named ..." is GAP'd: Conjure is
//!   Arena/Alchemy-only with no Effect variant.)
//!
//! NOTE: the Scryfall "Mill" / "Conjure" keyword entries are not
//! `KeywordAbility` variants and are reflected in the text only.

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
    let name = reg.interner_mut().intern("Syndicate Recruiter");
    let vampire = reg.interner_mut().intern("Vampire");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill_four,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_mill_four(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Then if there are five or more mana values among cards in your
    // graveyard, conjure a card named Dig Up the Body into your hand." —
    // Conjure is Arena/Alchemy-only; no Effect::Conjure variant.
    vec![Effect::Mill { player: trig.controller, count: 4 }]
}
