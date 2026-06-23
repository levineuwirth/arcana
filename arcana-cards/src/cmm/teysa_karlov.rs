//! Teysa Karlov — `{2}{W}{B}` 2/4 Legendary Human Advisor.
//!
//! Oracle:
//! * "If a creature dying causes a triggered ability of a permanent you
//!   control to trigger, that ability triggers an additional time." — a
//!   trigger-doubling rule modifier; no demonstrated primitive expresses
//!   altering how often another permanent's abilities trigger. GAP'd.
//! * "Creature tokens you control have vigilance and lifelink." — a
//!   filtered keyword anthem over token creatures you control, wired via
//!   the `SelfEntersBattlefield` install idiom: two
//!   `ContinuousEffect::filtered_keyword` installs (one per keyword), each
//!   lasting while Teysa is on the battlefield.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Teysa Karlov");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "If a creature dying causes a triggered ability of a permanent
    // you control to trigger, that ability triggers an additional time" —
    // a trigger-frequency rule modifier, not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_token_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "Creature tokens you control have vigilance and lifelink." — install
/// two filtered keyword anthems over token creatures you control.
fn install_token_anthem(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let token_creatures = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .tokens_only();
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                token_creatures.clone(),
                KeywordAbility::Vigilance,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                token_creatures,
                KeywordAbility::Lifelink,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
