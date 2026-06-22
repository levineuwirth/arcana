//! Glorfindel, Dauntless Rescuer — `{2}{G}` 3/2 Legendary Elf Noble.
//!
//! * Whenever you scry, choose one and Glorfindel gets +1/+1 until end of turn.
//!     • Glorfindel must be blocked this turn if able.
//!     • Glorfindel can't be blocked by more than one creature each combat
//!       this turn.
//!
//! There is no "whenever you scry" TriggerCondition variant, so the trigger
//! cannot be fired on its true event — the whole ability is GAP'd. The two
//! modal block-restriction riders also have no expressible Effect (no
//! "must be blocked if able" / "can't be blocked by more than one" effect).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glorfindel, Dauntless Rescuer");
    let elf = reg.interner_mut().intern("Elf");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — no "whenever you scry" TriggerCondition.
                //       Using your-upkeep as a closest-firing placeholder;
                //       effect GAP'd to Vec::new (modal block restrictions
                //       are also not expressible).
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_scry,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_scry(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: trigger fires on "whenever you scry" which has no variant; the
    //       +1/+1 plus the two modal block-restriction riders ("must be
    //       blocked if able", "can't be blocked by more than one creature")
    //       have no expressible effect/condition. Emit nothing.
    Vec::new()
}
