//! Bloodsworn Squire // Bloodsworn Knight — `{3}{B}` Vampire Soldier creature 3/3.
//! Front: {1}{B}, Discard a card: This creature gains indestructible until end of
//!   turn. Tap it. Then if there are four or more creature cards in your graveyard,
//!   transform this creature.
//! Back (Bloodsworn Knight): P/T equal to number of creature cards in graveyard.
//!   {1}{B}, Discard a card: This creature gains indestructible until end of turn.
//!   Tap it.
//!
//! GAP: Activated ability costs include "Discard a card" — discard as a cost is
//!   not expressible via OptionalPaymentKind (only Mana and Life supported).
//!   The activation and its conditional transform effect are not modeled.
//! Back face P/T each equal to the number of creature cards in your graveyard:
//!   wired as a Layer-7a self-CDA (`self_pt_cda`) installed on
//!   `SelfEntersBattlefield` with `Duration::WhileSourceShowsFace(1)` so it is
//!   dormant while the front face is up and lights up if the card transforms.
//!   Back-face bones are `PtValue::Star`.
//! GAP: Back face's own activated ability not modeled (back-face-only triggered
//!   ability not auto-installed on transform).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodsworn Squire");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire_sub);
    subtypes.0.insert(soldier_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Bloodsworn Knight");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vampire_sub);
    back_subtypes.0.insert(knight_sub);

    // Back face P/T = creature cards in your graveyard (a Layer-7a self-CDA,
    // installed on ETB and face-gated to the back face).
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Star),
            toughness: Some(PtValue::Star),
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: {1}{B}, Discard a card: gain indestructible + tap + conditional transform.
    // Discard-a-card cost is not expressible (OptionalPaymentKind supports only
    // Mana and Life). The activation is omitted entirely.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Install the back-face CDA on ETB; face-gated so it is dormant
            // while the (3/3 fixed) front face is up.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_back_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Layer 7a self-CDA: Bloodsworn Knight's P/T each equal to the number of
/// creature cards in your graveyard. Face-gated to the back face.
fn install_back_cda(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            creature_cards_in_your_graveyard,
            Duration::WhileSourceShowsFace(1),
        ),
    }]
}

/// P/T = the number of creature cards in your graveyard.
fn creature_cards_in_your_graveyard(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s
        .objects
        .objects_in_zone(Zone::Graveyard(who))
        .filter(|o| o.characteristics.types.is_creature())
        .count() as i32;
    (n, n)
}
