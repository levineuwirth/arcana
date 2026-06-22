//! Lazav, Dimir Mastermind — `{U}{U}{B}{B}` 3/3 Legendary Creature — Shapeshifter with
//! Hexproof.
//! "Whenever a creature card is put into an opponent's graveyard from anywhere, you may
//!  have Lazav become a copy of that card, except its name is Lazav, Dimir Mastermind,
//!  it's legendary in addition to its other types, and it has hexproof and this
//!  ability."
//!
//! The "become a copy of that graveyard card (with overrides)" effect is GAP'd:
//! CopyPermanent mints a new token copy of a battlefield permanent, not an in-place
//! self-copy of a card in a graveyard.

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
    let name = reg.interner_mut().intern("Lazav, Dimir Mastermind");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}{B}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::CREATURE.into())
                        .controlled_by(ControllerConstraint::Opponent),
                    from: None,
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: on_creature_to_gy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_creature_to_gy(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may have Lazav become a copy of that card, except …" — an in-place
    // self-becomes-a-copy of a card in a graveyard (with name/legendary/keyword
    // overrides) is not expressible (CopyPermanent mints a token copy of a battlefield
    // permanent).
    Vec::new()
}
