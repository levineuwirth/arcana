//! Runo Stromkirk // Krothuss, Lord of the Deep — {1}{U}{B}
//! Legendary Creature — Vampire Cleric // Legendary Creature — Kraken Horror (1/4)
//! Front: Flying.
//!   When Runo enters, put up to one target creature card from your graveyard on top of
//!   your library.
//!   At the beginning of your upkeep, look at the top card of your library. You may reveal
//!   that card. If a creature card with mana value 6 or greater is revealed this way, transform Runo.
//! Back: Flying.
//!   Whenever Krothuss attacks, create a tapped and attacking token that's a copy of another
//!   target attacking creature. If that creature is a Kraken, Leviathan, Octopus, or Serpent,
//!   create two of those tokens instead.
//! GAP: Front upkeep "look at the top card; if mv>=6 creature revealed, transform" — the
//!   intervening-if condition depends on a card reveal + card property check; not expressible.
//!   Transform trigger omitted.
//! GAP: Back-face "create a copy of target attacking creature" — CopyPermanent targets a
//!   permanent but creating it tapped+attacking is not separately modeled.
//! GAP: Back-face "if that creature is Kraken/Leviathan/Octopus/Serpent, create two" —
//!   conditional double-creation not expressible.
//! GAP: Back-face-only triggered ability not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Runo Stromkirk");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let cleric_sub = reg.interner_mut().intern("Cleric");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(vampire_sub);
    front_subtypes.0.insert(cleric_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Krothuss, Lord of the Deep");
    let kraken_sub = reg.interner_mut().intern("Kraken");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(kraken_sub);
    back_subtypes.0.insert(horror_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue() | ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(8)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: put up to one target creature card from graveyard on top of library
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_graveyard_to_top,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::creature(),
                        },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            }),
            // GAP: Upkeep "look at top card; if creature mv>=6 revealed, transform" not expressible.
            // GAP: Back-face attack trigger (copy attacking creature) not modeled.
    )
}

fn etb_graveyard_to_top(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::PutOnTopOfLibrary { target: *id }]
}
