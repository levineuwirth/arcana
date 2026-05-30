//! Ludevic, Necrogenius // Olag, Ludevic's Hubris — `{U}{B}` Legendary
//! Creature — Human Wizard 2/3.
//!
//! Front face:
//! - Whenever Ludevic enters or attacks, mill a card.
//! - {X}{U}{U}{B}{B}, Exile X creature cards from your graveyard: Transform
//!   Ludevic. X can't be 0. Activate only as a sorcery.
//!
//! Back face: Olag, Ludevic's Hubris — Legendary Creature — Zombie 4/4 (B/U).
//! "As this creature transforms into Olag, it becomes a copy of a creature
//! card exiled with it, except its name is Olag, it's 4/4, legendary blue and
//! black Zombie, with +1/+1 counters equal to the number of creature cards
//! exiled."
//!
//! # GAPs
//! - Transform activated ability costs "Exile X creature cards from your
//!   graveyard" — not expressible via OptionalPaymentKind (Mana/Life only).
//!   Transform activation is omitted.
//! - "Becomes a copy of a creature card exiled with it" on transform:
//!   back-face static copy mechanic is not expressible — Olag is wired as a
//!   static 4/4 Legendary Zombie (U/B).
//! - +1/+1 counters equal to number of creature cards exiled: count of exiled
//!   cards from cost is not tracked; counters not modeled.
//! - GAP: back-face-only triggered ability not modeled.

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
    let name = reg.interner_mut().intern("Ludevic, Necrogenius");
    let human_sub = reg.interner_mut().intern("Human");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(wizard_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Olag, Ludevic's Hubris");
    let zombie_sub = reg.interner_mut().intern("Zombie");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(zombie_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue() | ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // "Whenever Ludevic enters" — ETB trigger
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: ludevic_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "or attacks" — attacks trigger
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: ludevic_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn ludevic_mill(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Mill {
        player: trig.controller,
        count: 1,
    }]
}
