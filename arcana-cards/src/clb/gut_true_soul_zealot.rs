//! Gut, True Soul Zealot — `{2}{R}` 2/2 Legendary Goblin Shaman.
//!
//! Oracle:
//! * "Whenever you attack, you may sacrifice another creature or an
//!   artifact. If you do, create a 4/1 black Skeleton creature token
//!   with menace that's tapped and attacking." — the only attack-shaped
//!   trigger available is `CreatureAttacks { filter }`, which fires per
//!   attacking creature you control, not once per declared attack. More
//!   importantly the "you may sacrifice another creature or an artifact.
//!   If you do, …" gate is an optional SACRIFICE cost paid during
//!   resolution, which the `OptionalPayment` primitive does not model
//!   (only Mana / Life). With no way to make the token contingent on the
//!   sacrifice, the effect body is GAP'd to avoid an unconditional token.
//! * "Choose a Background" — a commander deck-building keyword with no
//!   rules-text effect; not in the usable keyword surface. GAP'd.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Gut, True Soul Zealot");
    let goblin = reg.interner_mut().intern("Goblin");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Choose a Background" — commander deck-building keyword; no
    // in-game rules effect and not in the usable keyword surface.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: attack_sacrifice,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_sacrifice(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice another creature or an artifact. If you do,
    // create a 4/1 black Skeleton with menace tapped and attacking" — the
    // optional SACRIFICE cost during resolution is not expressible via
    // OptionalPayment (Mana/Life only); without it the token cannot be
    // made contingent on the sacrifice.
    Vec::new()
}
