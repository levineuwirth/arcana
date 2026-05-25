//! Abstract Paintmage — `{U}{U/R}{R}` 2/2 red-blue Djinn Sorcerer.
//! "At the beginning of your first main phase, add {U}{R}. Spend
//! this mana only to cast instant and sorcery spells."
//! GAP: mana-spending restriction ("only to cast instant/sorcery")
//! is not expressible in the engine mana effect; plain AddMana emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Abstract Paintmage");
    let djinn = reg.interner_mut().intern("Djinn");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);
    subtypes.0.insert(sorcerer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U/R}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: main_phase_add_mana,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn main_phase_add_mana(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: mana-spending restriction (only for instant/sorcery spells) not expressible
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Blue, trig.source),
            ManaUnit::plain(ManaColor::Red, trig.source),
        ],
    }]
}
