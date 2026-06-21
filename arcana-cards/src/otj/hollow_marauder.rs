//! Hollow Marauder — `{6}{B}` 4/2 Specter Rogue with Flying.
//!
//! 1. "This spell costs {1} less to cast for each creature card in your
//!    graveyard." — dynamic cost reduction; GAP.
//! 2. Flying — base keyword, expressible.
//! 3. "When this creature enters, any number of target opponents each
//!    discard a card. For each of those opponents who didn't discard a
//!    card with mana value 4 or greater, draw a card." — GAP: the
//!    per-target discard with a conditional-draw payoff keyed on the
//!    discarded card's mana value is inexpressible with the demonstrated
//!    primitives.

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
    let name = reg.interner_mut().intern("Hollow Marauder");
    let specter = reg.interner_mut().intern("Specter");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(specter);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "This spell costs {1} less to cast for each creature card in
    // your graveyard" — no dynamic cost-reduction primitive.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_discard_and_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_discard_and_draw(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: any-number-of-target-opponents discard, then draw a card for
    // each opponent who didn't discard a mana-value-4+ card — the
    // conditional draw keyed on the discarded card's mana value is
    // inexpressible.
    Vec::new()
}
