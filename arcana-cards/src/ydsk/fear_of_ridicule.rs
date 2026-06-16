//! Fear of Ridicule — `{1}{B}{B}` 2/3 black Enchantment Creature —
//! Nightmare.
//!
//! Oracle:
//! * "Enchantment creatures you control have menace." — GAP: a static
//!   continuous anthem (grant menace to a board-wide set); not a
//!   triggered/activated ability and not expressible here. Omitted.
//! * "Whenever one or more enchantment creatures you control deal
//!   combat damage to a player, exile a random creature card from that
//!   player's library. Create a token that's a copy of that card,
//!   except it's a 1/1 enchantment creature." — GAP: the random-exile-
//!   from-library then token-copy-as-modified (1/1 enchantment) chain
//!   is not expressible (`CopyPermanent` copies a permanent/board
//!   object, not a randomly-exiled library card, and there is no
//!   random-library-exile-to-token primitive). The trigger SHAPE is
//!   recorded (DamageDealt by enchantment-creatures-you-control to a
//!   player, combat-only) but the effect returns `Vec::new()`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fear of Ridicule");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .with_types(TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE)),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: exile_random_and_copy,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn exile_random_and_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile a RANDOM creature card from that player's library, then
    // create a 1/1 enchantment-creature token copy of it. No primitive
    // for random-library-exile-to-modified-token-copy.
    Vec::new()
}
