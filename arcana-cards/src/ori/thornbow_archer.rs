//! Thornbow Archer — `{B}` 1/2 black Elf Archer.
//! "Whenever this creature attacks, each opponent who doesn't control an Elf
//! loses 1 life."
//!
//! GAP: "each opponent who doesn't control an Elf" — conditional per-opponent
//! filter (checking whether that opponent has an Elf) requires per-player
//! board inspection. Using script::opponents + script::count_matching to
//! approximate: subtract those who have an Elf.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thornbow Archer");
    let elf = reg.interner_mut().intern("Elf");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(archer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attacks,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attacks(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    opponents
        .into_iter()
        .filter_map(|p| {
            let elf_filter = script::subtype_filter(reg, "Elf")
                .controlled_by(ControllerConstraint::Any);
            let elves = script::count_matching(state, &elf_filter, p);
            if elves == 0 {
                Some(Effect::LoseLife { player: p, amount: 1 })
            } else {
                None
            }
        })
        .collect()
}
