//! Graveyard Trespasser // Graveyard Glutton — `{2}{B}` Human Werewolf 3/3 (front).
//!
//! Front face (Graveyard Trespasser):
//!   Ward—Discard a card. (GAP: non-mana ward not expressible.)
//!   Whenever this creature enters or attacks, exile up to one target card from a graveyard.
//!   If a creature card was exiled this way, each opponent loses 1 life and you gain 1 life.
//!   Daybound. (GAP: Daybound/Nightbound day-night cycle not modeled.)
//!
//! Back face (Graveyard Glutton) — Werewolf:
//!   Ward—Discard a card. (GAP: non-mana ward not expressible.)
//!   Whenever this creature enters or attacks, exile up to two target cards from graveyards.
//!   For each creature card exiled this way, each opponent loses 1 life and you gain 1 life.
//!   Nightbound. (GAP: Daybound/Nightbound day-night cycle not modeled.)
//!
//! GAP: Ward—Discard a card is a non-mana ward cost; not expressible (only mana/life supported).
//! GAP: Daybound/Nightbound day-night cycle and transform triggers not modeled.
//! GAP: "If a creature card was exiled this way" — exile-type tracking not in engine;
//!      life loss/gain fires unconditionally when a card is exiled.
//! GAP: Up to two targets (UpTo(2)) would need two separate ExileFromGraveyard effects;
//!      the target count is UpTo(1) on both faces here for the front; back extends to UpTo(1)
//!      (engine limitation: each TargetRequirement with UpTo(1) fires once per chosen card).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Graveyard Trespasser");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: Ward—Discard a card — non-mana ward not expressible; keywords: vec![]
        keywords: vec![],
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Back face: Graveyard Glutton — Werewolf
    let back_name = reg.interner_mut().intern("Graveyard Glutton");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            // GAP: Ward—Discard a card — non-mana ward not expressible
            keywords: vec![],
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Pre-intern graveyard target zone player placeholder
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: whenever this creature enters or attacks, exile up to one target card
            // from a graveyard. (ETB arm)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_graveyard_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::new(),
                        },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            })
            // Front face: whenever this creature attacks, exile up to one target card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: exile_graveyard_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::new(),
                        },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            }),
        // GAP: Daybound/Nightbound transform triggers not modeled (day-night cycle deferred).
        // GAP: back-face-only triggered abilities (enters/attacks with UpTo 2 targets) not
        //      auto-installed on transform.
    )
}

/// Exile the targeted card from a graveyard. Then (unconditionally) each opponent loses 1 life
/// and you gain 1 life.
/// GAP: "if a creature card was exiled this way" type-check not expressible; fires unconditionally.
fn exile_graveyard_card(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "if a creature card was exiled this way" type-check not performed; fires unconditionally.
    let mut effects = vec![Effect::ExileFromGraveyard { target: *id }];
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::LoseLife { player: opp, amount: 1 });
    }
    effects.push(Effect::GainLife { player: trig.controller, amount: 1 });
    effects
}
