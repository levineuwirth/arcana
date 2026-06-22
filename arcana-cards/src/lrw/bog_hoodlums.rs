//! Bog Hoodlums — `{5}{B}` 4/1 Goblin Warrior.
//! "This creature can't block."
//! "When this creature enters, clash with an opponent. If you win, put a
//! +1/+1 counter on this creature."
//!
//! Clash is not in the usable keyword surface and is unmodeled. The
//! "can't block" static is a continuous self-restriction with no
//! trigger/cost and is GAP'd. The ETB body needs the clash result to gate
//! the counter, which is not expressible, so it is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: "This creature can't block." — a static continuous self-restriction
// with no trigger/cost; not expressible in the MultiAbilityCreature
// surface.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bog Hoodlums");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: clash_on_enter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn clash_on_enter(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "clash with an opponent. If you win, put a +1/+1 counter on
    // this creature." — Clash is not a modeled mechanic and there is no
    // way to read the clash result to gate the counter, so the body is
    // omitted.
    Vec::new()
}
