//! Kiora, Sovereign of the Deep — `{3}{G}{U}` 4/5 Legendary Merfolk Noble.
//! Vigilance, ward {3}.
//! "Whenever you cast a Kraken, Leviathan, Octopus, or Serpent spell from your
//! hand, look at the top X cards of your library, where X is that spell's mana
//! value. You may cast a spell with mana value less than X from among them
//! without paying its mana cost. Put the rest on the bottom of your library in
//! a random order." The "from your hand" zone restriction is GAP'd (no
//! cast-zone filter on SpellCast); the impulsive free-cast-from-top-X-by-the-
//! cast-spell's-mana-value payoff is GAP'd (dynamic dig-depth keyed off the
//! triggering spell's mana value with an mv-gated free cast is not expressible).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kiora, Sovereign of the Deep");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(noble);

    let kraken = reg.interner_mut().intern("Kraken");
    let leviathan = reg.interner_mut().intern("Leviathan");
    let octopus = reg.interner_mut().intern("Octopus");
    let serpent = reg.interner_mut().intern("Serpent");
    let sea_filter = ObjectFilter::new()
        .with_subtypes_any(vec![kraken, leviathan, octopus, serpent]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Ward(ManaCost::parse("{3}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "from your hand" — no cast-zone filter on SpellCast.
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(sea_filter),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: sea_dig,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn sea_dig(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: look at the top X cards (X = triggering spell's mana value), cast a
    // spell with mana value < X for free, bottom the rest in random order.
    // Dynamic dig-depth keyed to the triggering spell's mana value plus an
    // mv-gated free cast is not expressible with the documented effect surface.
    Vec::new()
}
