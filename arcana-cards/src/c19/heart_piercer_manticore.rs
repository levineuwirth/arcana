//! Heart-Piercer Manticore — `{2}{R}{R}` 4/3 Manticore.
//! * "When this creature enters, you may sacrifice another creature.
//!   When you do, this creature deals damage equal to that creature's
//!   power to any target."
//! * "Embalm {5}{R}" (graveyard token-copy reanimation).
//!
//! GAP: Embalm is not in the usable keyword surface — emitted as no
//! keyword.
//!
//! The ETB ability is a reflexive trigger: an optional "sacrifice
//! another creature" followed by "when you do, deal damage equal to that
//! creature's power to any target". The damage amount is dynamic on the
//! SACRIFICED creature's power, which is gone by resolution and not
//! exposed to the trigger effect (same limitation as Airdrop Condor), and
//! the reflexive optional-sacrifice / "when you do" structure has no
//! single primitive. The whole ETB effect is therefore GAP'd; the
//! trigger + any-target requirement are still emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heart-Piercer Manticore");
    let manticore = reg.interner_mut().intern("Manticore");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(manticore);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Embalm {5}{R} is not in the usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_sacrifice_then_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }),
    )
}

fn etb_sacrifice_then_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reflexive "you may sacrifice another creature. When you do, deal
    // damage equal to that creature's power to any target". The optional
    // sacrifice + reflexive "when you do" structure has no single primitive,
    // and the damage amount (the sacrificed creature's power) is not available
    // at resolution (the creature is already gone). Whole effect GAP'd.
    Vec::new()
}
