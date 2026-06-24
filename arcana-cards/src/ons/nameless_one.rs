//! Nameless One — `{3}{U}` */* Wizard Avatar.
//!
//! Oracle:
//! * "Nameless One's power and toughness are each equal to the number
//!   of Wizards on the battlefield." — a characteristic-defining
//!   ability; P/T are marked as `*` (PtValue::Star). Wired at Layer 7a
//!   via `ContinuousEffect::self_pt_from_match` on a
//!   `SelfEntersBattlefield` trigger. The filter is uncontrolled
//!   (Wizards on the battlefield = both players' Wizards).
//! * "Morph {2}{U}." — the morph keyword with its mana cost.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nameless One");
    let wizard = reg.interner_mut().intern("Wizard");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wizard);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Morph(
            ManaCost::parse("{2}{U}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Layer 7a self-CDA: P/T each equal to the number of Wizards on the
/// battlefield (all players — no controller constraint).
fn install_cda(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Wizard");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match(
            trig.source,
            filter,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
