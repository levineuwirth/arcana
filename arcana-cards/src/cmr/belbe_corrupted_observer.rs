//! Belbe, Corrupted Observer — `{B}{G}` black/green Legendary Creature—Phyrexian
//! Zombie Elf. 2/2.
//! "At the beginning of each postcombat main phase, the active player adds
//! {C}{C} for each of your opponents who lost life this turn."

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Belbe, Corrupted Observer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let zombie = reg.interner_mut().intern("Zombie");
    let elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(zombie);
    subtypes.0.insert(elf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::PostCombatMain,
                whose: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: add_mana_per_opponent_lost_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn add_mana_per_opponent_lost_life(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // "the active player" — for a PhaseBegins trigger this is state.active_player().
    let active = state.active_player();
    // Count opponents of Belbe's controller who lost life this turn.
    let n: u32 = script::opponents(state, trig.controller)
        .into_iter()
        .filter(|&opp| script::life_lost_this_turn(state, opp) > 0)
        .count() as u32;
    if n == 0 {
        return Vec::new();
    }
    // Add {C}{C} for each such opponent → 2*n colorless mana total.
    let mana: Vec<ManaUnit> = (0..2 * n)
        .map(|_| ManaUnit::plain(ManaColor::Colorless, trig.source))
        .collect();
    vec![Effect::AddMana { player: active, mana }]
}
