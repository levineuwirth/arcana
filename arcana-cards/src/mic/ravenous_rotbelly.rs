//! Ravenous Rotbelly — `{4}{B}` 4/5 black Zombie Horror. "When this
//! creature enters, you may sacrifice up to three Zombies. When you
//! sacrifice one or more Zombies this way, each opponent sacrifices
//! that many creatures of their choice."
//!
//! The player-chosen "you may sacrifice up to three Zombies" clause is
//! modeled with `ChooseAnyNumberFromZone` (a min-0 / max-all pick over
//! Zombies you control, with `PickAction::Sacrifice`). The dependent
//! second clause ("each opponent sacrifices THAT MANY creatures") needs
//! to read the count of Zombies actually sacrificed this way and feed it
//! into a per-opponent sacrifice — that cross-clause dynamic count is not
//! expressible with the catalog, so it is GAP'd.

use arcana_core::effects::{Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ravenous Rotbelly");
    let zombie = reg.interner_mut().intern("Zombie");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_sacrifice_zombies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_sacrifice_zombies(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may sacrifice up to three Zombies" — a player-chosen
    // variable-count sacrifice over Zombies you control.
    let filter = script::subtype_filter(reg, "Zombie")
        .controlled_by(ControllerConstraint::You);
    vec![Effect::ChooseAnyNumberFromZone {
        chooser: trig.controller,
        zone: Zone::Battlefield,
        filter,
        action: PickAction::Sacrifice,
    }]
    // GAP: the "up to three" cap and the dependent second clause ("each
    // opponent sacrifices THAT MANY creatures of their choice", where the
    // count equals the Zombies sacrificed this way) are not expressible —
    // the catalog has no way to read the chosen-count and feed it into a
    // per-opponent sacrifice.
}
