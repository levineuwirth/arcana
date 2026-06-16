//! Sanctifier en-Vec — `{W}{W}` 2/2 white Human Cleric.
//! "Protection from black and from red"
//! "When this creature enters, exile all cards that are black or red from
//! all graveyards."
//! "If a black or red permanent, spell, or card not on the battlefield would
//! be put into a graveyard, exile it instead."
//!
//! Decomposition:
//! * Keyword line: Protection from black and from red. `Protection` is NOT in
//!   the usable `KeywordAbility` surface for this card class, so `keywords` is
//!   empty.
//!   // GAP: static keyword — Protection from black and from red.
//! * Trigger 1 — `When this creature enters, exile all cards that are black
//!   or red from all graveyards.` This is a zone-wide, color-filtered mass
//!   exile from every graveyard; the demonstrated effect surface only offers
//!   single-target `Effect::ExileFromGraveyard { target: id }` with no shown
//!   way to enumerate graveyard cards by color, so the effect body is GAP'd.
//! * Static replacement — `If a black or red permanent, spell, or card not on
//!   the battlefield would be put into a graveyard, exile it instead.` This is
//!   a continuous replacement effect with no trigger word and no cost; not
//!   expressible with the demonstrated API.
//!   // GAP: replacement — black/red cards put into a graveyard are exiled instead.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Sanctifier en-Vec");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Protection from black and from red — not in the usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_exile_graveyards,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "exile all cards that are black or red from all graveyards."
fn etb_exile_graveyards(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no demonstrated effect enumerates and exiles all color-matching
    // cards across every graveyard (only single-target ExileFromGraveyard exists).
    Vec::new()
}
