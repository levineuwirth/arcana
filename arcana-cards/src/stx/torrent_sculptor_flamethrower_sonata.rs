//! Torrent Sculptor // Flamethrower Sonata — MDFC.
//!
//! Front: {2}{U}{U} Creature — Merfolk Wizard 2/2, Ward {2}.
//! When this creature enters, exile an instant or sorcery card from your
//! graveyard. Put a number of +1/+1 counters on this creature equal to
//! half that card's mana value, rounded up.
//!
//! Back: Flamethrower Sonata — Sorcery.
//! Discard a card, then draw a card. When you discard an instant or sorcery
//! card this way, Flamethrower Sonata deals damage equal to that card's mana
//! value to target creature or planeswalker you don't control.
//!
//! GAP: ETB "exile instant/sorcery from graveyard; put +1/+1 counters = half
//! exiled card's MV rounded up" — cannot access CMC of the exiled card at
//! resolution; entire ETB counter effect is GAP'd.
//! GAP: Back face "discard a card, then draw; when you discard an instant/sorcery
//! this way, deal damage equal to its MV" — triggered damage from discard is not
//! expressible; back face spell_ability emits Discard + Draw only.
//! GAP: Back face's own triggered ability (deal damage on discard) is not modeled
//! (back-face-only triggered ability engine debt).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Torrent Sculptor");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Flamethrower Sonata");
    let back_chars = Characteristics {
        name: back_name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: Some(SpellAbilityDef {
            text: "Discard a card, then draw a card.".into(),
            target_requirements: vec![],
            modal: None,
            effect: flamethrower_sonata_resolve,
        }),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            .with_mdfc_back(back_face),
        // GAP: back-face-only triggered ability (deal damage when instant/sorcery
        // discarded this way) not modeled.
    )
}

fn etb_exile_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile an instant or sorcery card from your graveyard; put +1/+1
    // counters equal to half that card's mana value, rounded up" — targeted
    // graveyard exile + dynamic CMC-based counter count not expressible.
    vec![]
}

fn flamethrower_sonata_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Discard a card, then draw a card.
    // GAP: "when you discard an instant or sorcery this way, deal damage equal
    // to its MV to target creature or planeswalker you don't control" is a
    // triggered ability on the back face — not modeled.
    vec![
        Effect::Discard {
            player: entry.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards {
            player: entry.controller,
            count: 1,
        },
    ]
}
