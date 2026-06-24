//! Phyrexian Broodstar — `{6}{U}{U}` */* Phyrexian Beast with Flying.
//! "Affinity for Phyrexians (This spell costs {1} less to cast for each
//!  Phyrexian you control.)
//!  Flying
//!  Phyrexian Broodstar's power and toughness are each equal to the number
//!  of Phyrexians you control."
//!
//! Flying is a keyword. Affinity is a cost-reduction keyword with no
//! expressible variant — GAP. The characteristic-defining "power and
//! toughness each equal to the number of Phyrexians you control" is
//! installed as a Layer 7a self-CDA on ETB (`self_pt_from_match`); P/T
//! kept as `*` (PtValue::Star) in the bones.

// GAP (keyword): Affinity for Phyrexians — cost reduction not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Phyrexian Broodstar");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // */* — power and toughness are characteristic-defined (= Phyrexians
        // you control), installed as a Layer 7a self-CDA on ETB (id 1 below).
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Flying],
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

/// Layer 7a self-CDA: P/T each equal to the number of Phyrexians you control.
fn install_cda(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Phyrexian")
        .controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match(
            trig.source,
            filter,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
