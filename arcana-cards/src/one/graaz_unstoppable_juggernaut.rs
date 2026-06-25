//! Graaz, Unstoppable Juggernaut — `{8}` 7/5 Legendary Artifact Creature —
//! Juggernaut.
//! "Juggernauts you control attack each combat if able.
//!  Juggernauts you control can't be blocked by Walls.
//!  Other creatures you control have base power and toughness 5/3 and are
//!  Juggernauts in addition to their other creature types."
//!
//! The board-wide must-attack static is installed on ETB as a
//! `filtered_must_attack` over Juggernauts the controller controls (CR
//! 508.1a). The remaining two statics are GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Graaz, Unstoppable Juggernaut");
    let juggernaut = reg.interner_mut().intern("Juggernaut");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(juggernaut);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: static — "Juggernauts you control can't be blocked by Walls."
    // GAP: static — "Other creatures you control have base power and
    //       toughness 5/3 and are Juggernauts in addition to their other
    //       creature types." (board-wide base-PT-set + subtype-add static).
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_juggernauts_must_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn install_juggernauts_must_attack(
    _s: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let juggernaut = reg.interner().lookup("Juggernaut").unwrap_or_default();
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_must_attack(
            trig.source,
            ObjectFilter::creature()
                .with_subtype_sym(juggernaut)
                .controlled_by(ControllerConstraint::You),
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
