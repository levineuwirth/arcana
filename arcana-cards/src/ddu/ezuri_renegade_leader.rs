//! Ezuri, Renegade Leader — `{1}{G}{G}` 2/2 Legendary Creature — Elf Warrior.
//!
//! Oracle:
//! * {G}: Regenerate another target Elf.
//! * {2}{G}{G}{G}: Elf creatures you control get +3/+3 and gain trample until end of turn.
//!
//! Both are activated abilities. The first regenerates a chosen Elf
//! (targeted). "another" — the engine can't exclude the source from the
//! target filter, so any Elf is targetable; minor fidelity note. The second
//! is a non-targeted board pump over every Elf you control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ezuri, Renegade Leader");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let elf_target = ObjectFilter::creature().with_subtype_sym(elf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}: Regenerate another target Elf.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(elf_target),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: regenerate_elf,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}{G}{G}: Elf creatures you control get +3/+3 and gain trample until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}{G}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: overrun_elves,
            }),
    )
}

fn regenerate_elf(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Regenerate { target: *id }]
}

fn overrun_elves(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let elf = match reg.interner().lookup("Elf") {
        Some(sym) => sym,
        None => return Vec::new(),
    };
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(elf);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    ids.into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Trample],
        })
        .collect()
}
