//! Splinter & Leo, Father & Son — `{2}{W}` 2/2 Legendary Creature —
//! Mutant Ninja Rat Turtle.
//!
//! "When Splinter & Leo enter, choose one or both. Each mode must target a
//!  different player.
//!  • Target player creates a 2/2 red Mutant creature token.
//!  • Put a +1/+1 counter on each other creature target player controls."
//!
//! This is a MODAL triggered ability ("choose one or both"). The modal
//! machinery (ModalSpec / dispatch_modal_effect / with_mode_effects) is only
//! shown for spell abilities, not triggered abilities, and there's no way to
//! attach per-mode target requirements to a `TriggeredAbilityDef`. The ETB
//! trigger shape is recorded with a GAP'd payload.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Splinter & Leo, Father & Son");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let rat = reg.interner_mut().intern("Rat");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(rat);
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose one or both" modal ETB with per-mode player targets — no
    // modal triggered-ability machinery (modal dispatch is spell-only) and no
    // way to attach per-mode target requirements to a trigger.
    Vec::new()
}
