//! Valki, God of Lies // Tibalt, Cosmic Impostor — MDFC.
//!
//! Front: {1}{B} Legendary Creature — God 2/1.
//! When Valki enters, each opponent reveals their hand. For each opponent,
//! exile a creature card they revealed this way until Valki leaves the battlefield.
//! {X}: Choose a creature card exiled with Valki with mana value X. Valki becomes
//! a copy of that card.
//!
//! Back: Legendary Planeswalker — Tibalt, starting loyalty 5.
//! As Tibalt enters, you get an emblem: "You may play cards exiled with Tibalt,
//! Cosmic Impostor, and you may spend mana as though it were any color to cast
//! those spells."
//! +2: Exile the top card of each player's library.
//! −3: Exile target artifact or creature.
//! −8: Exile all graveyards. Add {R}{R}{R}.
//!
//! GAP: ETB "exile a creature card from each opponent's revealed hand until Valki
//!   leaves" — no Duration variant for "until source leaves battlefield" exile;
//!   effect emits Vec::new().
//! GAP: {X} activated ability "Valki becomes a copy of an exiled card" — the
//!   {X} cost itself is now expressible, but there is no Effect::BecomesCopy
//!   taking an exiled-card reference (and the ETB exile-until-leaves that
//!   populates the choice pool is also unmodeled), so the whole ability is
//!   GAP'd (cost left unwired).
//! GAP: Tibalt emblem — no Effect::CreateEmblem; GAP'd.
//! GAP: Tibalt +2/−3/−8 loyalty abilities — planeswalker loyalty ability shape
//!   not yet in engine API; GAP'd (back-face-only activated abilities not modeled).
//! GAP: −8 "exile all graveyards, add {R}{R}{R}" partially expressible but not
//!   wirable as loyalty ability; GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Valki, God of Lies");
    let god_sub = reg.interner_mut().intern("God");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(god_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Tibalt, Cosmic Impostor — Legendary Planeswalker — Tibalt
    let back_name = reg.interner_mut().intern("Tibalt, Cosmic Impostor");
    let tibalt_sub = reg.interner_mut().intern("Tibalt");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(tibalt_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: Some(ManaCost::parse("{5}{B}{R}").expect("valid cost")),
            colors: ColorSet::black() | ColorSet::red(),
            types: TypeLine::PLANESWALKER.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            loyalty: Some(5),
            ..Default::default()
        },
        spell_ability: None,
        // GAP: Tibalt ETB emblem, +2, -3, -8 loyalty abilities not modeled
        // (planeswalker loyalty ability shape not in engine API; back-face-only).
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            // Front face ETB: reveal each opponent's hand, exile a creature card from each.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: valki_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
        // GAP: {X} activated ability not modeled (no variant for copy-of-exiled-card).
        // GAP: back-face-only triggered/activated abilities not auto-installed.
    )
}

fn valki_etb(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each opponent reveals their hand; for each opponent, exile a creature
    // card they revealed this way until Valki leaves the battlefield" — no
    // Duration::UntilSourceLeavesBattlefield for conditional exile; returning
    // empty.
    vec![]
}
