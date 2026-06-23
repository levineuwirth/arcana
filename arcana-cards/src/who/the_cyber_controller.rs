//! The Cyber-Controller — `{X}{U}{U}{B}` 3/3 Legendary Artifact Creature — Cyberman.
//!
//! When The Cyber-Controller enters, each opponent mills X cards. Put
//! all creature cards milled this way onto the battlefield face down
//! under your control. They're 2/2 Cyberman artifact creatures.
//! Other artifact creatures you control get +1/+1.
//!
//! The ETB mills X (the X paid for the spell), but a PendingTrigger
//! exposes no x_value accessor and no script helper yields the X paid,
//! so the mill amount is uncomputable; the "put all creature cards
//! milled this way onto the battlefield face down as 2/2 Cybermen"
//! rider is likewise inexpressible. Both GAP'd.
//!
//! "Other artifact creatures you control get +1/+1" IS expressible: a
//! `SelfEntersBattlefield` trigger installs a layer-7c filtered pump
//! (`ContinuousEffect::filtered_pump`) over artifact creatures you
//! control, lasting while this creature is on the battlefield. The
//! filter matches base characteristics, so this creature (an artifact
//! creature) self-includes — a documented minor fidelity gap on the
//! word "Other".

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
    let name = reg.interner_mut().intern("The Cyber-Controller");
    let cyberman = reg.interner_mut().intern("Cyberman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cyberman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{U}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: ETB "each opponent mills X" — no x_value accessor on a trigger to
    //      recover the X paid; "put creature cards milled this way onto the
    //      battlefield face down as 2/2 Cybermen" is unexpressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_artifact_anthem,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Install "(other) artifact creatures you control get +1/+1", anchored to
/// this creature and lasting while it remains on the battlefield.
fn install_artifact_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
        .controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
