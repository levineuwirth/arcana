//! Galadhrim Brigade — `{2}{G}` 2/2 Elf Soldier.
//!
//! Oracle:
//! * "Squad {1}{G}" — the Squad keyword. NOT a usable `KeywordAbility`
//!   variant in this engine's keyword surface, so the keyword line is
//!   empty. The Squad ETB rider ("When this creature enters, create
//!   that many tokens that are copies of it") is gated on the
//!   additional-cost squad payment count, which is not tracked by any
//!   demonstrated primitive — GAP'd.
//! * "Other Elves you control get +1/+1." — a static continuous anthem,
//!   now wired as a `SelfEntersBattlefield` trigger that installs a
//!   `ContinuousEffect::filtered_pump` over Elves you control
//!   (glorious_anthem idiom). NOTE: `filtered_pump` matches base
//!   characteristics, so the "OTHER" exclusion isn't expressed — the
//!   Brigade is itself an Elf and self-includes (documented minor gap).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Galadhrim Brigade");
    let elf = reg.interner_mut().intern("Elf");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: keyword "Squad {1}{G}" — not an expressible KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: Squad ETB rider "create that many token copies of it" — the
    // count depends on how many times the squad additional cost was
    // paid, which no demonstrated primitive tracks.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_elf_anthem,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// ETB: install "Elves you control get +1/+1" anchored to the Brigade,
/// lasting while it is on the battlefield.
fn install_elf_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter =
        script::subtype_filter(reg, "Elf").controlled_by(ControllerConstraint::You);
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
