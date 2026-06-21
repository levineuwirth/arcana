//! Labyrinth Guardian — `{1}{U}` 2/3 Creature — Illusion Warrior.
//!
//! * "When this creature becomes the target of a spell, sacrifice it."
//!   — SelfBecomesTarget trigger; resolved as the controller
//!   sacrificing a Labyrinth Guardian (the only expressible "sacrifice
//!   this" is a name-filtered Effect::Sacrifice).
//! * Embalm {3}{U} — not in the usable keyword surface; the
//!   graveyard-activated embalm (cost + token-copy with altered
//!   characteristics) is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Labyrinth Guardian");
    let illusion = reg.interner_mut().intern("Illusion");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: Embalm {3}{U} — not in the usable keyword surface; the
    // graveyard-activated token-copy-with-altered-characteristics is
    // not expressible.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: sacrifice_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn sacrifice_self(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let nm = reg.interner().lookup("Labyrinth Guardian");
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter { name: nm, ..ObjectFilter::default() },
        count: 1,
    }]
}
