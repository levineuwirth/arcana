//! Ruthless Winnower — `{3}{B}{B}` 4/4 black Elf Rogue. "At the beginning of each
//! player's upkeep, that player sacrifices a non-Elf creature of their choice."
//! StepBegins(Upkeep, Any); "that player" is the upkeep owner — the active
//! player while the trigger resolves (`state.active_player()`).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ruthless Winnower");
    let elf = reg.interner_mut().intern("Elf");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: on_each_upkeep,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_each_upkeep(
    state: &GameState,
    _trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "That player" = whose upkeep it is = the active player. The upkeep
    // owner sacrifices a non-Elf creature ("Elf" interned in register; on
    // a failed lookup skip the exclusion).
    let mut non_elf_filter = ObjectFilter::creature();
    if let Some(elf) = reg.interner().lookup("Elf") {
        non_elf_filter = non_elf_filter.without_subtype_sym(elf);
    }
    vec![Effect::Sacrifice {
        player: state.active_player(),
        filter: non_elf_filter,
        count: 1,
    }]
}
