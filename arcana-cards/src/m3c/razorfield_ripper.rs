//! Razorfield Ripper — `{2}{W}` 3/3 Artifact Creature — Equipment Rhino.
//! Whenever this creature or equipped creature attacks, you get {E} (an
//! energy counter), then it gets +X/+X until end of turn, where X is the
//! amount of {E} you have.
//! Reconfigure—Pay {2} or {E}{E}{E}.

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Razorfield Ripper");
    let equipment = reg.interner_mut().intern("Equipment");
    let rhino = reg.interner_mut().intern("Rhino");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    subtypes.0.insert(rhino);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Reconfigure (Scryfall-parsed) is not in the usable keyword
        // surface and its attach/unattach activation is not expressible.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // "this creature or equipped creature attacks" — only the self side
            // is expressible (SelfAttacks). The equipped-creature side has no
            // variant in the demonstrated API (partial).
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: gain_energy_then_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gain_energy_then_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you get {E}" — expressible. The "+X/+X where X is the amount of {E} you
    // have" rider is GAP'd: there is no script:: accessor for a player's energy
    // total, so the dynamic X cannot be computed with the allowed helpers.
    vec![Effect::GainEnergy {
        player: trig.controller,
        amount: 1,
    }]
}
