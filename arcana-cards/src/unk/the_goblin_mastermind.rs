//! The Goblin Mastermind — `{2}{B}{R}` 3/3 Legendary Goblin Wizard.
//! "As long as you control The Goblin Mastermind or it's your commander,
//!  permanents you control are Kindred Goblins in addition to their other
//!  types. ..." (a static type-grant — not expressible; GAP'd.)
//! "Whenever The Goblin Mastermind deals combat damage to a player, you
//!  create a token that's a copy of one of Skirk Prospector, Goblin Lackey,
//!  Goblin Piledriver, Goblin Warchief, Boggart Harbinger, or Goblin
//!  Ringleader, chosen at random."
//!
//! The combat-damage trigger structure is preserved, but minting a token
//! that's a copy of a named card chosen at random is not expressible
//! (Effect::CopyPermanent copies an existing permanent by id; there is no
//! copy-card-by-name), so its effect is GAP'd. The static type-grant is also
//! GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Goblin Mastermind");
    let goblin = reg.interner_mut().intern("Goblin");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "permanents/spells/cards you control are Kindred Goblins in
    // addition to their other types" — a board-wide type-grant is not
    // expressible with the demonstrated API.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: copy_random_goblin,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn copy_random_goblin(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
    // GAP: "create a token that's a copy of one of [six named cards] chosen at
    // random" — minting a copy of a named card is not expressible
    // (Effect::CopyPermanent copies an existing permanent by id; no
    // copy-card-by-name path).
}
