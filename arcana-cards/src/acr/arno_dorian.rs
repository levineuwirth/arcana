//! Arno Dorian — `{2}{B}{R}` 3/3 Legendary Human Assassin.
//! Deathtouch.
//! Other Assassins you control get +2/+0. (static anthem — wired)
//! Disguise {B}{R} (not in the usable keyword surface — GAP)
//!
//! The "Other Assassins you control get +2/+0" static is modeled as a
//! `SelfEntersBattlefield` trigger that installs a `ContinuousEffect::
//! filtered_pump` over Assassins you control, lasting while Arno is on the
//! battlefield (glorious_anthem idiom). NOTE: `filtered_pump` matches the
//! filter against base characteristics, so the "OTHER" exclusion isn't
//! expressed — Arno is itself an Assassin and so self-includes; a
//! documented minor fidelity gap.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: Disguise {B}{R} — Disguise is not among the implemented
// KeywordAbility variants; emitted as keywords: [Deathtouch] only.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arno Dorian");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_assassin_anthem,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// ETB: install "Assassins you control get +2/+0" anchored to Arno,
/// lasting while it is on the battlefield.
fn install_assassin_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Assassin")
        .controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            2,
            0,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
