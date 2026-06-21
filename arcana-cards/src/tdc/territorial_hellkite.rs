//! Territorial Hellkite — `{2}{R}{R}` 6/5 Dragon.
//! Flying, haste.
//! At the beginning of combat on your turn, choose an opponent at random
//! that this creature didn't attack during your last combat. This
//! creature attacks that player this combat if able. If you can't choose
//! an opponent this way, tap this creature. (effect GAP'd — no
//! must-attack-specific-player / last-combat-history primitive.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Territorial Hellkite");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: forced_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn forced_attack(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose an opponent at random that this creature didn't attack
    // last combat; it must attack that player this combat, else tap it."
    // No primitive forces an attack against a specific player, and no
    // last-combat attack-history accessor exists.
    Vec::new()
}
