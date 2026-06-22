//! Tanuki Transplanter — `{3}{G}` 2/4 Artifact Creature — Equipment Dog.
//! Whenever this creature or equipped creature attacks, add an amount of {G} equal
//! to its power. Until end of turn, you don't lose this mana as steps and phases
//! end.
//! Reconfigure {3}.
//!
//! The attack trigger adds {G} equal to this creature's power (the "or equipped
//! creature" half requires equipment-relative attacker tracking and the
//! mana-persistence rider are fidelity GAPs). Reconfigure is not an available
//! keyword/cost shape — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tanuki Transplanter");
    let equipment = reg.interner_mut().intern("Equipment");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: add_green_for_power,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP keyword/cost: "Reconfigure {3}" — not an available cost shape.
    )
}

fn add_green_for_power(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP fidelity: "or equipped creature attacks" half, and the "you don't lose
    // this mana as steps and phases end" persistence rider.
    let n = script::power_of(state, trig.source).max(0) as usize;
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, trig.source); n],
    }]
}
