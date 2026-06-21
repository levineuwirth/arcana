//! Thundertrap Trainer — `{1}{U}` 1/2 Creature — Otter Wizard. Mono-blue.
//!
//! Oracle:
//! - "Offspring {4}" — GAP: Offspring is not a usable `KeywordAbility`; the
//!   additional-cost + token-copy ETB rider is unexpressible.
//! - "When this creature enters, look at the top four cards of your library.
//!   You may reveal a noncreature, nonland card from among them and put it into
//!   your hand. Put the rest on the bottom of your library in a random order."
//!   — ETB `DigTopN` of 4, filter = noncreature/nonland, rest BottomRandom.

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
    let name = reg.interner_mut().intern("Thundertrap Trainer");
    let otter = reg.interner_mut().intern("Otter");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(otter);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: "Offspring {4}" keyword (additional cost + token-copy ETB) is not
        // a usable KeywordAbility.
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_dig(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::default()
        .without_types(TypeLine(TypeLine::CREATURE | TypeLine::LAND));
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 4,
        filter: Some(filter),
        rest: DigRest::BottomRandom,
    }]
}
