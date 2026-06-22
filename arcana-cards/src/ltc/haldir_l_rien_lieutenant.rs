//! Haldir, Lórien Lieutenant — `{X}{G}` 0/0 Legendary Elf Soldier.
//! Haldir enters with X +1/+1 counters on it.
//! Vigilance.
//! {5}{G}: Until end of turn, other Elves you control gain vigilance
//! and get +1/+1 for each +1/+1 counter on Haldir.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Haldir, Lórien Lieutenant");
    let elf = reg.interner_mut().intern("Elf");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::CountersFromX {
                kind: CounterKind::PlusOnePlusOne,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{G}: Until end of turn, other Elves you control gain vigilance and get +1/+1 for each +1/+1 counter on Haldir.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: buff_other_elves,
            }),
    )
}

fn buff_other_elves(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let n = state
        .objects
        .get(ctx.source)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne)) as i32;
    let ids: Vec<_> = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Elf").controlled_by(ControllerConstraint::You),
        ctx.controller,
    )
    .into_iter()
    .filter(|id| *id != ctx.source)
    .collect();
    ids.into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: n,
            toughness: n,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Vigilance],
        })
        .collect()
}
