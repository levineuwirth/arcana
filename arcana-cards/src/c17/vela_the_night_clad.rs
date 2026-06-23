//! Vela the Night-Clad — `{4}{U}{B}` 4/4 Legendary Human Wizard with Intimidate.
//!
//! "Other creatures you control have intimidate." — WIRED as an ETB-installed
//!  `ContinuousEffect::keyword_anthem(Intimidate)`, lasting while Vela is on the
//!  battlefield (glorious_anthem precedent). The anthem grants intimidate to all
//!  your creatures; the "OTHER" self-inclusion is the documented minor fidelity
//!  gap (Vela already has intimidate as a base keyword anyway).
//! "Whenever Vela or another creature you control leaves the battlefield, each
//!  opponent loses 1 life." — ZoneChange requires a single concrete `to` zone;
//!  "leaves the battlefield" (to any of graveyard/hand/exile/library) is not
//!  expressible, and there is no SelfLeavesBattlefield trigger variant (GAP).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vela the Night-Clad");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Intimidate],
        ..Default::default()
    };

    // GAP: "Whenever Vela or another creature you control leaves the battlefield,
    //   each opponent loses 1 life" — ZoneChange requires a single concrete `to`
    //   zone; "leaves the battlefield" (to any zone) is not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            // "Other creatures you control have intimidate." installed on ETB.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_intimidate_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Install "creatures you control have intimidate" anchored to Vela, lasting
/// while it remains on the battlefield.
fn install_intimidate_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::keyword_anthem(
            trig.source,
            trig.controller,
            KeywordAbility::Intimidate,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
