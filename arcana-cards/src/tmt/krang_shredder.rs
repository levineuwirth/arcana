//! Krang & Shredder — `{4}{U/B}{U/B}` 6/7 Legendary Utrom Human Ninja.
//! Whenever Krang & Shredder enter or attack, each opponent exiles cards from
//! the top of their library until they exile a nonland card.
//! Disappear — At the beginning of your end step, if a permanent left the
//! battlefield under your control this turn, you may cast a card exiled with
//! Krang & Shredder without paying its mana cost.
//!
//! Abilities:
//!  - "enter or attack" → two triggered abilities (SelfEntersBattlefield and
//!    SelfAttacks). The effect "each opponent exiles from the top of their
//!    library until they exile a nonland card" is not expressible: the
//!    reveal/exile-until primitives (RevealUntil) operate on the controller's
//!    OWN library to hand/battlefield, not "each opponent exiles to a
//!    Krang-linked exile zone" → both trigger bodies are GAP'd (the trigger
//!    structure is recorded; the effect is dropped).
//!  - "Disappear — At the beginning of your end step, …" is a self-linked-exile
//!    free-cast ability (intervening-if on "a permanent left the battlefield
//!    under your control this turn", plus casting a card exiled-with-this from a
//!    bespoke linked exile zone). Neither the gate nor the cast-from-linked-exile
//!    is expressible → GAP'd.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Krang & Shredder");
    let utrom = reg.interner_mut().intern("Utrom");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(utrom);
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U/B}{U/B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: opponents_exile_until_nonland,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: opponents_exile_until_nonland,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP (third ability): "Disappear — At the beginning of your end step, if
        // a permanent left the battlefield under your control this turn, you may
        // cast a card exiled with Krang & Shredder without paying its mana cost."
        // The "permanent left the battlefield this turn" gate, the
        // exiled-with-this linked exile zone, and the free-cast-from-that-zone are
        // all unexpressible with the demonstrated surface.
    )
}

fn opponents_exile_until_nonland(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each opponent exiles cards from the top of their library until they
    // exile a nonland card." — the reveal/exile-until primitives target the
    // controller's own library (to hand/battlefield); there is no
    // each-opponent exile-from-top-until-nonland (into a Krang-linked exile
    // zone) primitive.
    Vec::new()
}
