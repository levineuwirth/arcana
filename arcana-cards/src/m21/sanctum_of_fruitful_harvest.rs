//! Sanctum of Fruitful Harvest — `{2}{G}` Legendary Enchantment — Shrine.
//! "At the beginning of your first main phase, add X mana of any one
//! color, where X is the number of Shrines you control."

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
use arcana_core::types::{
    CardId, ColorSet, ManaColor, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sanctum of Fruitful Harvest");
    let shrine = reg.interner_mut().intern("Shrine");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shrine);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: harvest_mana,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…add X mana of any one color, where X is the number of Shrines you
/// control."
fn harvest_mana(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let shrines = script::subtype_filter(reg, "Shrine")
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &shrines, trig.controller);
    if n == 0 {
        return Vec::new();
    }
    // GAP: "of any one color" — the color choice is not modeled; adds
    // green (this Shrine's color) deterministically.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Green, trig.source);
            n as usize
        ],
    }]
}
