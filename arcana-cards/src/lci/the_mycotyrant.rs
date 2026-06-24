//! The Mycotyrant — `{1}{B}{G}` */* Legendary Elder Fungus with Trample.
//! "The Mycotyrant's power and toughness are each equal to the number of
//! creatures you control that are Fungi and/or Saprolings." — installed as a
//! Layer 7a self-CDA on ETB via `ContinuousEffect::self_pt_from_match` over a
//! Fungus-or-Saproling creature filter (PtValue::Star marks the bones).
//! "At the beginning of your end step, create X 1/1 black Fungus creature
//! tokens with 'This token can't block,' where X is the number of times
//! you descended this turn."

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
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Mycotyrant");
    let fungus = reg.interner_mut().intern("Fungus");
    // Interned so the CDA filter's lookup of "Saproling" always resolves.
    let _saproling = reg.interner_mut().intern("Saproling");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_make_fungi,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // CDA: P/T each equal to the Fungus/Saproling creatures you control.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Layer 7a self-CDA: P/T each equal to the number of creatures you control
/// that are Fungi and/or Saprolings.
fn install_cda(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let syms: Vec<_> = ["Fungus", "Saproling"]
        .iter()
        .filter_map(|s| reg.interner().lookup(s))
        .collect();
    let filter = ObjectFilter::creature()
        .with_subtypes_any(syms)
        .controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match(
            trig.source,
            filter,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn end_step_make_fungi(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "create X 1/1 black Fungus tokens ..., where X is the number of
    // times you descended this turn." There is no descend-count script
    // helper, so the dynamic X is uncomputable; the whole effect is omitted
    // rather than emit a wrong fixed count. (The token's "can't block" rider
    // is also a static the TokenDefinition can't carry.)
    Vec::new()
}
