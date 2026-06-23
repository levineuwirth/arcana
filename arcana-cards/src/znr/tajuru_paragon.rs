//! Tajuru Paragon — `{1}{G}` 3/2 green Elf.
//! "Tajuru Paragon is also a Cleric, Rogue, Warrior, and Wizard." (static)
//! "Kicker {3}"
//! "When this creature enters, if it was kicked, reveal the top six cards
//! of your library. You may put a card that shares a creature type with it
//! from among them into your hand. Put the rest on the bottom of your
//! library in a random order."
//!
//! Kicker is not in the usable keyword surface (GAP'd). The type-adding
//! static ("is also a Cleric, Rogue, Warrior, and Wizard") is a pure
//! continuous ability — GAP'd. The ETB is wired as a top-6 dig with a
//! creature-card filter (DigTopN / BottomRandom), the closest expressible
//! form: the "if it was kicked" gate (no kicked predicate) and the
//! "shares a creature type with it" restriction (dynamic, this card's own
//! subtypes) are both GAP'd, so the dig fires unconditionally and offers
//! any creature card.

use arcana_core::effects::{DigRest, Effect};
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
    let name = reg.interner_mut().intern("Tajuru Paragon");
    let elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Kicker {3} — Kicker is not in the usable keyword surface.
        keywords: vec![],
        // GAP (static): "Tajuru Paragon is also a Cleric, Rogue, Warrior,
        // and Wizard." — a continuous type-adding ability, not a
        // triggered/activated ability.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "When this creature enters, if it was kicked, reveal the top
            // six cards ... You may put a card that shares a creature type
            // with it into your hand. Put the rest on the bottom in random
            // order."
            // GAP: the "if it was kicked" gate (no kicked predicate) and the
            // "shares a creature type with it" filter (dynamic self-subtypes)
            // are not expressible; wired as an unconditional top-6 dig for
            // any creature card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig_six,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_dig_six(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 6,
        filter: Some(ObjectFilter::new().with_types(TypeLine::CREATURE.into())),
        rest: DigRest::BottomRandom,
    }]
}
